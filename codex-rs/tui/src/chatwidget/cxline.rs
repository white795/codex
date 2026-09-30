//! CxLine runtime state and footer integration.

use std::path::Path;

use super::ChatWidget;
use crate::statusline::CxLineConfig;
use crate::statusline::StatusLineContext;
use crate::statusline::build_statusline;

pub(super) struct CxLineRuntime {
    config: CxLineConfig,
    has_saved_config: bool,
}

impl CxLineRuntime {
    pub(super) fn load(codex_home: &Path) -> Self {
        match CxLineConfig::load_saved(codex_home) {
            Ok(Some(config)) => Self {
                config,
                has_saved_config: true,
            },
            Ok(None) => Self {
                config: CxLineConfig::load(codex_home).unwrap_or_else(|error| {
                    tracing::warn!(%error, "Failed to load CxLine theme fallback; using built-in defaults");
                    CxLineConfig::default()
                }),
                has_saved_config: false,
            },
            Err(error) => {
                tracing::warn!(%error, "Failed to load saved CxLine configuration; using official status line");
                Self {
                    config: CxLineConfig::default(),
                    has_saved_config: false,
                }
            }
        }
    }

    pub(super) fn enabled_config(&self) -> Option<&CxLineConfig> {
        (self.has_saved_config && self.config.enabled).then_some(&self.config)
    }

    fn editor_config(&self) -> CxLineConfig {
        self.config.clone()
    }

    fn apply_editor_config(&mut self, config: CxLineConfig, codex_home: &Path) {
        let has_saved_config =
            self.has_saved_config || matches!(CxLineConfig::load_saved(codex_home), Ok(Some(_)));
        if has_saved_config {
            self.config = config;
            self.has_saved_config = true;
        }
    }
}

impl ChatWidget {
    pub(crate) fn cxline_editor_config(&self) -> CxLineConfig {
        self.cxline_runtime.editor_config()
    }

    pub(crate) fn apply_cxline_editor_config(&mut self, config: CxLineConfig) {
        self.cxline_runtime
            .apply_editor_config(config, &self.local_settings.codex_home);
        self.refresh_status_surfaces();
    }

    pub(super) fn cxline_enabled(&self) -> bool {
        self.cxline_runtime.enabled_config().is_some()
    }

    pub(super) fn refresh_cxline_status_line(&mut self) -> bool {
        let Some(config) = self.cxline_runtime.enabled_config() else {
            return false;
        };
        let (used_tokens, window_size) =
            self.token_info
                .as_ref()
                .map_or((None, self.config.model_context_window), |info| {
                    (
                        Some(info.last_token_usage.tokens_in_context_window()),
                        info.model_context_window,
                    )
                });
        let context = StatusLineContext::new(self.current_model(), self.status_line_cwd())
            .with_reasoning_effort(self.effective_reasoning_effort())
            .with_context(used_tokens, window_size);
        let line = build_statusline(config, &context).render_line();

        self.bottom_pane.set_status_line_enabled(/*enabled*/ true);
        self.set_status_line(Some(line));
        self.set_status_line_hyperlink(/*url*/ None);
        true
    }
}
