use std::collections::VecDeque;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;

use super::*;
use crate::statusline::CxLineConfig;
use crate::statusline::GitPreviewData;
use crate::workspace_command::WorkspaceCommand;
use crate::workspace_command::WorkspaceCommandError;
use crate::workspace_command::WorkspaceCommandExecutor;
use crate::workspace_command::WorkspaceCommandOutput;
use codex_protocol::openai_models::ReasoningEffort;
use pretty_assertions::assert_eq;

struct QueuedGitRunner {
    outputs: Mutex<VecDeque<WorkspaceCommandOutput>>,
    commands: Mutex<Vec<WorkspaceCommand>>,
}

impl QueuedGitRunner {
    fn new(outputs: impl IntoIterator<Item = WorkspaceCommandOutput>) -> Self {
        Self {
            outputs: Mutex::new(outputs.into_iter().collect()),
            commands: Mutex::new(Vec::new()),
        }
    }

    fn command_count(&self) -> usize {
        self.commands.lock().unwrap().len()
    }
}

impl WorkspaceCommandExecutor for QueuedGitRunner {
    fn run(
        &self,
        command: WorkspaceCommand,
    ) -> Pin<
        Box<dyn Future<Output = Result<WorkspaceCommandOutput, WorkspaceCommandError>> + Send + '_>,
    > {
        self.commands.lock().unwrap().push(command);
        let output = self
            .outputs
            .lock()
            .unwrap()
            .pop_front()
            .expect("queued Git output");
        Box::pin(async move { Ok(output) })
    }
}

struct GitPreviewUpdate {
    request_id: uuid::Uuid,
    cwd: PathBuf,
    preview: Option<GitPreviewData>,
}

fn git_status_output(
    branch: &str,
    status_record: &str,
    ahead: u32,
    behind: u32,
) -> WorkspaceCommandOutput {
    WorkspaceCommandOutput {
        exit_code: 0,
        stdout: format!(
            "# branch.oid abcdef\n# branch.head {branch}\n# branch.ab +{ahead} -{behind}\n{status_record}"
        ),
        stderr: String::new(),
    }
}

async fn next_git_preview_update(
    events: &mut tokio::sync::mpsc::UnboundedReceiver<AppEvent>,
) -> GitPreviewUpdate {
    loop {
        let event = tokio::time::timeout(std::time::Duration::from_secs(1), events.recv())
            .await
            .expect("CxLine Git preview timeout")
            .expect("CxLine Git preview channel closed");
        if let AppEvent::CxLineGitPreviewUpdated {
            request_id,
            cwd,
            preview,
        } = event
        {
            return GitPreviewUpdate {
                request_id,
                cwd,
                preview,
            };
        }
    }
}

fn save_cxline_config(chat: &ChatWidget, config: &CxLineConfig) {
    let root = chat.local_settings.codex_home.join("cxline");
    std::fs::create_dir_all(&root).expect("create CxLine config directory");
    std::fs::write(
        root.join("config.toml"),
        toml::to_string_pretty(config).expect("serialize CxLine config"),
    )
    .expect("save CxLine config");
}

fn paired_usage_snapshot(
    limit_id: &str,
    primary_used_percent: i32,
    secondary_used_percent: i32,
) -> RateLimitSnapshot {
    RateLimitSnapshot {
        limit_id: Some(limit_id.to_string()),
        limit_name: Some(limit_id.to_string()),
        primary: Some(RateLimitWindow {
            used_percent: primary_used_percent,
            window_duration_mins: Some(5 * 60),
            resets_at: None,
        }),
        secondary: Some(RateLimitWindow {
            used_percent: secondary_used_percent,
            window_duration_mins: Some(7 * 24 * 60),
            resets_at: None,
        }),
        ..snapshot(/*percent*/ 0.0)
    }
}

#[tokio::test]
async fn missing_cxline_config_keeps_the_official_status_line() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    chat.local_settings.tui.status_line = Some(vec!["hostname".to_string()]);

    chat.refresh_status_line();

    assert_eq!(status_line_text(&chat), codex_config::os_host_name());
}

#[tokio::test]
async fn saved_cxline_config_renders_live_model_directory_and_context() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.current_cwd = Some(std::path::PathBuf::from("/workspace/cxline-demo"));
    chat.set_reasoning_effort(Some(ReasoningEffort::High));
    chat.set_token_info(Some(make_token_info(
        /*total_tokens*/ 64_000, /*context_window*/ 128_000,
    )));

    chat.apply_cxline_editor_config(config);

    let line = status_line_text(&chat).expect("CxLine footer");
    assert!(line.contains("GPT 5.2 Codex · high"));
    assert!(line.contains("cxline-demo"));
    assert!(line.contains("50% · 64.0k tokens"));
    insta::assert_snapshot!("cxline_live_base_data", line);
}

#[tokio::test]
async fn cxline_model_segment_tracks_effective_fast_mode() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.4")).await;
    set_chatgpt_auth(&mut chat);
    set_fast_mode_test_catalog(&mut chat);
    let mut config = CxLineConfig::default();
    config.segments.directory.enabled = false;
    config.segments.git.enabled = false;
    config.segments.context.enabled = false;
    config.segments.usage.enabled = false;
    save_cxline_config(&chat, &config);
    chat.set_reasoning_effort(Some(ReasoningEffort::XHigh));
    chat.apply_cxline_editor_config(config);

    chat.set_service_tier(Some(ServiceTier::Fast.request_value().to_string()));
    assert_eq!(
        status_line_text(&chat),
        Some("\u{e26d} GPT 5.4 · xhigh fast".to_string())
    );
    insta::assert_snapshot!(
        "cxline_live_fast_mode",
        status_line_text(&chat).expect("CxLine footer with Fast mode")
    );

    chat.set_service_tier(Some(SERVICE_TIER_DEFAULT_REQUEST_VALUE.to_string()));
    assert_eq!(
        status_line_text(&chat),
        Some("\u{e26d} GPT 5.4 · xhigh".to_string())
    );
}

#[tokio::test]
async fn disabled_saved_cxline_config_restores_the_official_status_line() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    chat.local_settings.tui.status_line = Some(vec!["hostname".to_string()]);
    let config = CxLineConfig {
        enabled: false,
        ..CxLineConfig::default()
    };
    save_cxline_config(&chat, &config);

    chat.apply_cxline_editor_config(config);

    assert_eq!(status_line_text(&chat), codex_config::os_host_name());
}

#[test]
fn saved_cxline_config_is_available_during_runtime_startup() {
    let home = tempfile::tempdir().expect("temporary CODEX_HOME");
    let root = home.path().join("cxline");
    std::fs::create_dir_all(&root).expect("create CxLine config directory");
    let saved = CxLineConfig::default();
    std::fs::write(
        root.join("config.toml"),
        toml::to_string_pretty(&saved).expect("serialize CxLine config"),
    )
    .expect("save CxLine config");

    let runtime = crate::chatwidget::cxline::CxLineRuntime::load(home.path());

    assert_eq!(runtime.enabled_config(), Some(&saved));
}

#[tokio::test]
async fn cxline_uses_only_codex_rate_limits_and_tracks_window_updates() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.current_cwd = Some(std::path::PathBuf::from("/workspace/cxline-demo"));
    chat.apply_cxline_editor_config(config);

    chat.on_rate_limit_snapshot(Some(paired_usage_snapshot(
        "codex_other",
        /*primary_used_percent*/ 91,
        /*secondary_used_percent*/ 88,
    )));
    assert!(
        !status_line_text(&chat)
            .expect("CxLine footer")
            .contains("91%")
    );

    chat.on_rate_limit_snapshot(Some(paired_usage_snapshot(
        "codex", /*primary_used_percent*/ 25, /*secondary_used_percent*/ 40,
    )));
    let initial = status_line_text(&chat).expect("CxLine footer with usage");
    assert!(initial.contains("25%"));
    insta::assert_snapshot!("cxline_live_rate_limits", initial);

    chat.on_rate_limit_snapshot(Some(paired_usage_snapshot(
        "codex", /*primary_used_percent*/ 63, /*secondary_used_percent*/ 77,
    )));
    let updated = status_line_text(&chat).expect("updated CxLine footer");
    assert!(updated.contains("63%"));
    assert!(!updated.contains("25%"));
}

#[tokio::test]
async fn cxline_weekly_only_usage_uses_the_weekly_percent_and_reset_label() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: Some("codex".to_string()),
        limit_name: Some("codex".to_string()),
        primary: Some(RateLimitWindow {
            used_percent: 42,
            window_duration_mins: Some(7 * 24 * 60),
            resets_at: Some(1_800_000_000),
        }),
        secondary: None,
        ..snapshot(/*percent*/ 0.0)
    }));
    let reset_label = chat
        .rate_limit_snapshots_by_limit_id
        .get("codex")
        .and_then(|snapshot| snapshot.primary.as_ref())
        .and_then(|window| window.resets_at.as_deref())
        .expect("localized weekly reset label");

    let line = status_line_text(&chat).expect("CxLine footer with weekly usage");

    assert!(line.contains(&format!("42% · {reset_label}")));
}

#[tokio::test]
async fn cxline_git_preview_runs_through_the_workspace_runner_and_renders_the_result() {
    let (mut chat, mut events, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let cwd = test_path_buf("/workspace/repo");
    let runner = Arc::new(QueuedGitRunner::new([git_status_output(
        "feature/cxline",
        "u UU N... 100644 100644 100644 100644 a b c file.rs\n",
        /*ahead*/ 3,
        /*behind*/ 2,
    )]));
    chat.current_cwd = Some(cwd.clone());
    chat.workspace_command_runner = Some(runner.clone());
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);

    chat.apply_cxline_editor_config(config);
    let update = next_git_preview_update(&mut events).await;

    assert_eq!(update.cwd, cwd);
    assert!(chat.set_cxline_git_preview(update.request_id, update.cwd, update.preview));
    let line = status_line_text(&chat).expect("CxLine footer with Git preview");
    assert!(line.contains("feature/cxline"));
    assert!(line.contains("⚠ ↑3 ↓2"));
    assert_eq!(runner.command_count(), 1);
    insta::assert_snapshot!("cxline_live_git_preview", line);
}

#[tokio::test]
async fn cxline_git_preview_rejects_stale_cwd_and_request_identity_then_refreshes() {
    let (mut chat, mut events, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let old_cwd = test_path_buf("/workspace/old");
    let new_cwd = test_path_buf("/workspace/new");
    let runner = Arc::new(QueuedGitRunner::new([
        git_status_output("old-branch", "", /*ahead*/ 0, /*behind*/ 0),
        git_status_output("new-branch", "", /*ahead*/ 0, /*behind*/ 0),
        git_status_output(
            "refreshed-branch",
            "? generated.txt\n",
            /*ahead*/ 1,
            /*behind*/ 0,
        ),
        WorkspaceCommandOutput {
            exit_code: 128,
            stdout: String::new(),
            stderr: "not a git repository".to_string(),
        },
    ]));
    chat.current_cwd = Some(old_cwd.clone());
    chat.workspace_command_runner = Some(runner.clone());
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    let old_update = next_git_preview_update(&mut events).await;

    chat.current_cwd = Some(new_cwd.clone());
    chat.refresh_status_line();
    let new_update = next_git_preview_update(&mut events).await;

    assert_eq!(old_update.cwd, old_cwd);
    assert_eq!(new_update.cwd, new_cwd);
    assert_ne!(old_update.request_id, new_update.request_id);
    assert!(!chat.set_cxline_git_preview(
        old_update.request_id,
        old_update.cwd,
        old_update.preview
    ));
    assert!(!chat.set_cxline_git_preview(
        uuid::Uuid::nil(),
        new_update.cwd.clone(),
        Some(GitPreviewData {
            branch: "wrong-request".to_string(),
            status: "✓".to_string(),
            ahead: 0,
            behind: 0,
        }),
    ));
    assert!(chat.set_cxline_git_preview(new_update.request_id, new_update.cwd, new_update.preview));
    assert!(
        status_line_text(&chat)
            .expect("current CxLine Git preview")
            .contains("new-branch")
    );

    chat.request_status_line_branch_refresh();
    let refreshed = next_git_preview_update(&mut events).await;

    assert_ne!(refreshed.request_id, new_update.request_id);
    assert!(chat.set_cxline_git_preview(refreshed.request_id, refreshed.cwd, refreshed.preview));
    let line = status_line_text(&chat).expect("refreshed CxLine Git preview");
    assert!(line.contains("refreshed-branch"));
    assert!(line.contains("● ↑1"));

    chat.request_status_line_branch_refresh();
    let failed = next_git_preview_update(&mut events).await;
    assert!(failed.preview.is_none());
    assert!(chat.set_cxline_git_preview(failed.request_id, failed.cwd, failed.preview));
    assert!(
        !status_line_text(&chat)
            .expect("CxLine footer after failed Git refresh")
            .contains("refreshed-branch")
    );
    assert_eq!(runner.command_count(), 4);
}

#[tokio::test]
async fn disabled_cxline_git_segment_does_not_start_a_workspace_command() {
    let (mut chat, _events, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let runner = Arc::new(QueuedGitRunner::new([]));
    chat.workspace_command_runner = Some(runner.clone());
    let mut config = CxLineConfig::default();
    config.segments.git.enabled = false;
    save_cxline_config(&chat, &config);

    chat.apply_cxline_editor_config(config);
    tokio::task::yield_now().await;

    assert_eq!(runner.command_count(), 0);
}

#[tokio::test]
async fn cxline_git_refresh_preserves_a_terminal_title_branch_refresh() {
    let (mut chat, _events, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    let runner = Arc::new(QueuedGitRunner::new([
        git_status_output("cxline-branch", "", /*ahead*/ 0, /*behind*/ 0),
        git_status_output("title-branch", "", /*ahead*/ 0, /*behind*/ 0),
    ]));
    chat.workspace_command_runner = Some(runner.clone());
    chat.local_settings.tui.terminal_title = Some(vec!["git-branch".to_string()]);

    chat.request_status_line_branch_refresh();
    tokio::task::yield_now().await;
    tokio::task::yield_now().await;

    assert!(chat.status_line_branch_pending);
    assert_eq!(runner.command_count(), 2);
}
