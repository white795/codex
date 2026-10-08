//! CxLine saved configuration and editor handoff for the current widget.
//!
//! The configuration UI edits its own draft; only its exit baseline is returned to the widget.
//! Theme fallback alone must not opt a session into CxLine before a main configuration is saved.

use std::path::Path;

use super::ChatWidget;
use crate::statusline::CxLineConfig;

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
}

#[cfg(test)]
#[path = "cxline_config_tests.rs"]
mod config_tests;
