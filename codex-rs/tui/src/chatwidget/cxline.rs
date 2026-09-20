//! CxLine runtime state and footer integration.

use std::path::Path;
use std::path::PathBuf;

use super::ChatWidget;
use super::status_surfaces::five_hour_status_window;
use super::status_surfaces::weekly_status_window;
use crate::statusline::CxLineConfig;
use crate::statusline::GitPreviewData;
use crate::statusline::StatusLineContext;
use crate::statusline::build_statusline;
use uuid::Uuid;

#[derive(Default)]
struct CxLineRateLimits {
    five_hour_used_percent: Option<f64>,
    weekly_used_percent: Option<f64>,
    weekly_resets_at: Option<String>,
}

#[derive(Default)]
struct CxLineGitState {
    cwd: Option<PathBuf>,
    preview: Option<GitPreviewData>,
    pending_request_id: Option<Uuid>,
    lookup_complete: bool,
}

pub(super) struct CxLineRuntime {
    config: CxLineConfig,
    has_saved_config: bool,
    git: CxLineGitState,
}

impl CxLineRuntime {
    pub(super) fn load(codex_home: &Path) -> Self {
        match CxLineConfig::load_saved(codex_home) {
            Ok(Some(config)) => Self {
                config,
                has_saved_config: true,
                git: CxLineGitState::default(),
            },
            Ok(None) => Self {
                config: CxLineConfig::load(codex_home).unwrap_or_else(|error| {
                    tracing::warn!(%error, "Failed to load CxLine theme fallback; using built-in defaults");
                    CxLineConfig::default()
                }),
                has_saved_config: false,
                git: CxLineGitState::default(),
            },
            Err(error) => {
                tracing::warn!(%error, "Failed to load saved CxLine configuration; using official status line");
                Self {
                    config: CxLineConfig::default(),
                    has_saved_config: false,
                    git: CxLineGitState::default(),
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

    fn git_segment_enabled(&self) -> bool {
        self.enabled_config()
            .is_some_and(|config| config.segments.git.enabled)
    }

    fn sync_git_cwd(&mut self, cwd: &Path) {
        if self.git.cwd.as_deref() == Some(cwd) {
            return;
        }
        self.git.cwd = Some(cwd.to_path_buf());
        self.git.preview = None;
        self.git.pending_request_id = None;
        self.git.lookup_complete = false;
    }

    fn begin_git_lookup(&mut self, cwd: &Path) -> Option<Uuid> {
        self.sync_git_cwd(cwd);
        if self.git.pending_request_id.is_some() || self.git.lookup_complete {
            return None;
        }
        let request_id = Uuid::new_v4();
        self.git.pending_request_id = Some(request_id);
        Some(request_id)
    }

    fn invalidate_git_lookup(&mut self, cwd: &Path) {
        self.sync_git_cwd(cwd);
        self.git.pending_request_id = None;
        self.git.lookup_complete = false;
    }

    fn complete_git_without_runner(&mut self, cwd: &Path) {
        self.sync_git_cwd(cwd);
        self.git.preview = None;
        self.git.pending_request_id = None;
        self.git.lookup_complete = true;
    }

    fn clear_git(&mut self) {
        self.git.cwd = None;
        self.git.preview = None;
        self.git.pending_request_id = None;
        self.git.lookup_complete = false;
    }

    fn apply_git_preview(
        &mut self,
        request_id: Uuid,
        cwd: &Path,
        preview: Option<GitPreviewData>,
    ) -> bool {
        if self.git.cwd.as_deref() != Some(cwd) || self.git.pending_request_id != Some(request_id) {
            return false;
        }
        self.git.preview = preview;
        self.git.pending_request_id = None;
        self.git.lookup_complete = true;
        true
    }

    fn git_preview(&self, cwd: &Path) -> Option<&GitPreviewData> {
        if self.git.cwd.as_deref() == Some(cwd) {
            self.git.preview.as_ref()
        } else {
            None
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
        if !self.cxline_enabled() {
            self.cxline_runtime.clear_git();
            return false;
        }
        let cwd = self.status_line_cwd().to_path_buf();
        if self.cxline_runtime.git_segment_enabled() {
            self.ensure_cxline_git_preview(&cwd);
        } else {
            self.cxline_runtime.clear_git();
        }
        let rate_limits = self.cxline_rate_limits();
        let (used_tokens, window_size) =
            self.token_info
                .as_ref()
                .map_or((None, self.config.model_context_window), |info| {
                    (
                        Some(info.last_token_usage.tokens_in_context_window()),
                        info.model_context_window,
                    )
                });
        let mut context = StatusLineContext::new(self.current_model(), &cwd)
            .with_reasoning_effort(self.effective_reasoning_effort())
            .with_context(used_tokens, window_size)
            .with_rate_limit(
                rate_limits.five_hour_used_percent,
                rate_limits.weekly_used_percent,
                rate_limits.weekly_resets_at,
            );
        if let Some(preview) = self.cxline_runtime.git_preview(&cwd) {
            context = context.with_git_preview(
                &preview.branch,
                &preview.status,
                preview.ahead,
                preview.behind,
            );
        }
        let Some(config) = self.cxline_runtime.enabled_config() else {
            return false;
        };
        let line = build_statusline(config, &context).render_line();

        self.bottom_pane.set_status_line_enabled(/*enabled*/ true);
        self.set_status_line(Some(line));
        self.set_status_line_hyperlink(/*url*/ None);
        true
    }

    fn ensure_cxline_git_preview(&mut self, cwd: &Path) {
        let Some(runner) = self.workspace_command_runner.clone() else {
            self.cxline_runtime.complete_git_without_runner(cwd);
            return;
        };
        let Some(request_id) = self.cxline_runtime.begin_git_lookup(cwd) else {
            return;
        };
        let cwd = cwd.to_path_buf();
        let tx = self.app_event_tx.clone();
        tokio::spawn(async move {
            let preview = crate::statusline::collect_git_preview(runner.as_ref(), &cwd).await;
            tx.send(crate::app_event::AppEvent::CxLineGitPreviewUpdated {
                request_id,
                cwd,
                preview,
            });
        });
    }

    pub(super) fn request_cxline_git_preview_refresh(&mut self) {
        if !self.cxline_runtime.git_segment_enabled() {
            return;
        }
        let cwd = self.status_line_cwd().to_path_buf();
        self.cxline_runtime.invalidate_git_lookup(&cwd);
        self.ensure_cxline_git_preview(&cwd);
    }

    pub(crate) fn set_cxline_git_preview(
        &mut self,
        request_id: Uuid,
        cwd: PathBuf,
        preview: Option<GitPreviewData>,
    ) -> bool {
        if !self
            .cxline_runtime
            .apply_git_preview(request_id, &cwd, preview)
        {
            return false;
        }
        self.refresh_status_surfaces();
        true
    }

    fn cxline_rate_limits(&self) -> CxLineRateLimits {
        let Some(snapshot) = self.rate_limit_snapshots_by_limit_id.get("codex") else {
            return CxLineRateLimits::default();
        };
        let five_hour = five_hour_status_window(snapshot).map(|(window, _)| window);
        let weekly = weekly_status_window(snapshot).map(|(window, _)| window);

        CxLineRateLimits {
            five_hour_used_percent: five_hour.map(|window| window.used_percent),
            weekly_used_percent: weekly.map(|window| window.used_percent),
            weekly_resets_at: weekly.and_then(|window| window.resets_at.clone()),
        }
    }
}
