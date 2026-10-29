//! CxLine background Git lookups, cache lifecycle, and official terminal-title coexistence.

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

#[tokio::test]
async fn enabled_cxline_git_schedules_only_one_probe_while_the_lookup_is_pending() {
    let (mut chat, mut events, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let runner = Arc::new(QueuedGitRunner::new([git_status_output(
        "cached-branch",
        "",
        /*ahead*/ 0,
        /*behind*/ 0,
    )]));
    chat.current_cwd = Some(test_path_buf("/workspace/repo"));
    chat.workspace_command_runner = Some(runner.clone());
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);

    chat.apply_cxline_editor_config(config);
    chat.refresh_status_line();
    chat.refresh_status_line();
    tokio::task::yield_now().await;

    assert_eq!(runner.command_count(), 1);
    let update = next_git_preview_update(&mut events).await;
    assert!(chat.set_cxline_git_preview(update.request_id, update.cwd, update.preview));
    chat.refresh_status_line();
    chat.refresh_status_line();
    tokio::task::yield_now().await;

    assert_eq!(runner.command_count(), 1);
    assert!(
        status_line_text(&chat)
            .expect("cached CxLine Git preview")
            .contains("cached-branch")
    );
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

#[tokio::test]
async fn cxline_git_rejects_an_old_request_after_returning_to_the_same_directory() {
    let (mut chat, mut events, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let first_cwd = test_path_buf("/workspace/first");
    let runner = Arc::new(QueuedGitRunner::new([
        git_status_output("old-first", "", /*ahead*/ 0, /*behind*/ 0),
        git_status_output("second", "", /*ahead*/ 0, /*behind*/ 0),
        git_status_output("current-first", "", /*ahead*/ 0, /*behind*/ 0),
    ]));
    chat.current_cwd = Some(first_cwd.clone());
    chat.workspace_command_runner = Some(runner.clone());
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    let old = next_git_preview_update(&mut events).await;

    chat.current_cwd = Some(test_path_buf("/workspace/second"));
    chat.refresh_status_line();
    let second = next_git_preview_update(&mut events).await;
    chat.current_cwd = Some(first_cwd);
    chat.refresh_status_line();
    let current = next_git_preview_update(&mut events).await;

    assert_eq!(old.cwd, current.cwd);
    assert_ne!(old.request_id, current.request_id);
    assert!(!chat.set_cxline_git_preview(old.request_id, old.cwd, old.preview));
    assert!(!chat.set_cxline_git_preview(second.request_id, second.cwd, second.preview));
    assert!(chat.set_cxline_git_preview(current.request_id, current.cwd, current.preview));
    let line = status_line_text(&chat).expect("current directory's Git preview");
    assert!(line.contains("current-first"));
    assert!(!line.contains("old-first"));
    assert_eq!(runner.command_count(), 3);
}

#[tokio::test]
async fn disabling_cxline_or_its_git_segment_rejects_pending_results_before_reenable() {
    for disable_whole_footer in [false, true] {
        let (mut chat, mut events, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
        chat.local_settings.tui.status_line = Some(vec!["hostname".to_string()]);
        let runner = Arc::new(QueuedGitRunner::new([
            git_status_output("disabled-request", "", /*ahead*/ 0, /*behind*/ 0),
            git_status_output("reenabled-request", "", /*ahead*/ 0, /*behind*/ 0),
        ]));
        chat.workspace_command_runner = Some(runner.clone());
        let config = CxLineConfig::default();
        save_cxline_config(&chat, &config);
        chat.apply_cxline_editor_config(config.clone());
        let old = next_git_preview_update(&mut events).await;

        let mut disabled = config.clone();
        if disable_whole_footer {
            disabled.enabled = false;
        } else {
            disabled.segments.git.enabled = false;
        }
        chat.apply_cxline_editor_config(disabled);
        assert!(!chat.set_cxline_git_preview(old.request_id, old.cwd.clone(), old.preview.clone()));
        if disable_whole_footer {
            assert_eq!(status_line_text(&chat), codex_config::os_host_name());
        }
        chat.request_status_line_branch_refresh();
        tokio::task::yield_now().await;
        assert_eq!(runner.command_count(), 1);

        chat.apply_cxline_editor_config(config);
        let current = next_git_preview_update(&mut events).await;
        assert_ne!(old.request_id, current.request_id);
        assert!(!chat.set_cxline_git_preview(old.request_id, old.cwd, old.preview));
        assert!(chat.set_cxline_git_preview(current.request_id, current.cwd, current.preview));
        assert!(
            status_line_text(&chat)
                .expect("reenabled Git preview")
                .contains("reenabled-request")
        );
        assert_eq!(runner.command_count(), 2);
    }
}

#[tokio::test]
async fn failed_cxline_git_lookup_is_cached_until_an_explicit_refresh() {
    let (mut chat, mut events, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let runner = Arc::new(QueuedGitRunner::new([
        WorkspaceCommandOutput {
            exit_code: 128,
            stdout: String::new(),
            stderr: "not a git repository".to_string(),
        },
        git_status_output("recovered-branch", "", /*ahead*/ 0, /*behind*/ 0),
    ]));
    chat.workspace_command_runner = Some(runner.clone());
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    let failed = next_git_preview_update(&mut events).await;
    assert!(failed.preview.is_none());
    assert!(chat.set_cxline_git_preview(failed.request_id, failed.cwd, failed.preview));

    chat.refresh_status_line();
    chat.refresh_status_line();
    tokio::task::yield_now().await;
    assert_eq!(runner.command_count(), 1);

    chat.request_status_line_branch_refresh();
    let recovered = next_git_preview_update(&mut events).await;
    assert!(chat.set_cxline_git_preview(recovered.request_id, recovered.cwd, recovered.preview));
    assert!(
        status_line_text(&chat)
            .expect("recovered Git preview")
            .contains("recovered-branch")
    );
    assert_eq!(runner.command_count(), 2);
}

#[tokio::test]
async fn cxline_git_refreshes_after_turn_completion_and_interruption() {
    for interrupted in [false, true] {
        let (mut chat, mut events, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
        let runner = Arc::new(QueuedGitRunner::new([
            git_status_output("before-turn", "", /*ahead*/ 0, /*behind*/ 0),
            git_status_output(
                "after-turn",
                "? result.txt\n",
                /*ahead*/ 1,
                /*behind*/ 0,
            ),
        ]));
        chat.workspace_command_runner = Some(runner.clone());
        let config = CxLineConfig::default();
        save_cxline_config(&chat, &config);
        chat.apply_cxline_editor_config(config);
        let initial = next_git_preview_update(&mut events).await;
        assert!(chat.set_cxline_git_preview(initial.request_id, initial.cwd, initial.preview));

        if interrupted {
            handle_turn_interrupted(&mut chat, "turn-1");
        } else {
            handle_turn_completed(&mut chat, "turn-1", /*duration_ms*/ None);
        }
        let refreshed = next_git_preview_update(&mut events).await;
        assert!(chat.set_cxline_git_preview(
            refreshed.request_id,
            refreshed.cwd,
            refreshed.preview
        ));
        assert!(
            status_line_text(&chat)
                .expect("Git preview after the turn")
                .contains("after-turn ● ↑1")
        );
        assert_eq!(runner.command_count(), 2);
    }
}
