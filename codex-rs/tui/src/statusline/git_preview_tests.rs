use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Mutex;
use std::time::Duration;

use pretty_assertions::assert_eq;

use super::GitPreviewData;
use super::collect_git_preview;
use crate::workspace_command::WorkspaceCommand;
use crate::workspace_command::WorkspaceCommandError;
use crate::workspace_command::WorkspaceCommandExecutor;
use crate::workspace_command::WorkspaceCommandOutput;

struct RecordingRunner {
    output: WorkspaceCommandOutput,
    commands: Mutex<Vec<WorkspaceCommand>>,
}

impl RecordingRunner {
    fn new(exit_code: i32, stdout: &str) -> Self {
        Self {
            output: WorkspaceCommandOutput {
                exit_code,
                stdout: stdout.to_string(),
                stderr: String::new(),
            },
            commands: Mutex::new(Vec::new()),
        }
    }
}

impl WorkspaceCommandExecutor for RecordingRunner {
    fn run(
        &self,
        command: WorkspaceCommand,
    ) -> Pin<
        Box<dyn Future<Output = Result<WorkspaceCommandOutput, WorkspaceCommandError>> + Send + '_>,
    > {
        self.commands.lock().unwrap().push(command);
        let output = self.output.clone();
        Box::pin(async move { Ok(output) })
    }
}

#[tokio::test]
async fn git_preview_collects_conflicts_and_upstream_divergence_with_a_bounded_command() {
    let runner = RecordingRunner::new(
        /*exit_code*/ 0,
        "# branch.oid abcdef\n# branch.head feature/cxline\n# branch.upstream origin/feature/cxline\n# branch.ab +3 -2\nu UU N... 100644 100644 100644 100644 a b c file.rs\n? new.txt\n",
    );
    let cwd = PathBuf::from("/workspace/repo");

    let preview = collect_git_preview(&runner, &cwd).await;

    assert_eq!(
        preview,
        Some(GitPreviewData {
            branch: "feature/cxline".to_string(),
            status: "⚠".to_string(),
            ahead: 3,
            behind: 2,
        })
    );
    let commands = runner.commands.lock().unwrap();
    let command = commands.first().expect("one Git status command");
    let argv = command.argv.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(commands.len(), 1);
    assert_eq!(
        (
            argv,
            command.cwd.as_ref(),
            command.env.get("GIT_OPTIONAL_LOCKS").cloned(),
            command.timeout,
            command.output_bytes_cap,
            command.disable_output_cap,
        ),
        (
            vec![
                "git",
                "-c",
                codex_git_utils::SAFE_BARE_REPOSITORY_CONFIG,
                "status",
                "--porcelain=v2",
                "--branch",
            ],
            Some(&cwd),
            Some(Some("0".to_string())),
            Duration::from_secs(5),
            64 * 1024_usize,
            false,
        )
    );
}

#[tokio::test]
async fn git_preview_distinguishes_clean_dirty_and_detached_worktrees() {
    let clean = RecordingRunner::new(
        /*exit_code*/ 0,
        "# branch.oid abcdef\n# branch.head main\n",
    );
    let dirty = RecordingRunner::new(
        /*exit_code*/ 0,
        "# branch.oid abcdef\n# branch.head topic\n1 .M N... 100644 100644 100644 a b file.rs\n",
    );
    let detached = RecordingRunner::new(
        /*exit_code*/ 0,
        "# branch.oid abcdef\n# branch.head (detached)\n# branch.ab +0 -0\n",
    );
    let cwd = PathBuf::from("/workspace/repo");

    let previews = (
        collect_git_preview(&clean, &cwd).await,
        collect_git_preview(&dirty, &cwd).await,
        collect_git_preview(&detached, &cwd).await,
    );

    assert_eq!(
        previews,
        (
            Some(GitPreviewData {
                branch: "main".to_string(),
                status: "✓".to_string(),
                ahead: 0,
                behind: 0,
            }),
            Some(GitPreviewData {
                branch: "topic".to_string(),
                status: "●".to_string(),
                ahead: 0,
                behind: 0,
            }),
            Some(GitPreviewData {
                branch: "detached".to_string(),
                status: "✓".to_string(),
                ahead: 0,
                behind: 0,
            }),
        )
    );
}

#[tokio::test]
async fn git_preview_omits_non_git_or_failed_workspace_results() {
    let runner = RecordingRunner::new(/*exit_code*/ 128, "");

    assert_eq!(
        collect_git_preview(&runner, &PathBuf::from("/workspace/not-a-repo")).await,
        None
    );
}
