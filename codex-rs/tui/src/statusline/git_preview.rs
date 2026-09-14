//! Asynchronous Git metadata collection for the CxLine segment.
//!
//! The renderer stays command-free: this adapter executes one bounded porcelain-v2 status probe
//! through the active workspace runner, then reduces it to the five fields CxLine can display.

use std::path::Path;

use super::GitPreviewData;
use crate::workspace_command::WorkspaceCommand;
use crate::workspace_command::WorkspaceCommandExecutor;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum WorktreeStatus {
    #[default]
    Clean,
    Dirty,
    Conflicts,
}

pub(crate) async fn collect_git_preview(
    runner: &dyn WorkspaceCommandExecutor,
    cwd: &Path,
) -> Option<GitPreviewData> {
    let output = runner
        .run(
            WorkspaceCommand::new([
                "git",
                "-c",
                codex_git_utils::SAFE_BARE_REPOSITORY_CONFIG,
                "status",
                "--porcelain=v2",
                "--branch",
            ])
            .cwd(cwd.to_path_buf())
            .env("GIT_OPTIONAL_LOCKS", "0"),
        )
        .await
        .ok()?;
    if !output.success() {
        return None;
    }

    parse_porcelain_v2(&output.stdout)
}

fn parse_porcelain_v2(output: &str) -> Option<GitPreviewData> {
    let mut branch = None;
    let mut ahead = 0;
    let mut behind = 0;
    let mut status = WorktreeStatus::Clean;

    for line in output.lines() {
        if let Some(head) = line.strip_prefix("# branch.head ") {
            branch = Some(if head.starts_with('(') {
                "detached".to_string()
            } else {
                head.to_string()
            });
        } else if let Some(divergence) = line.strip_prefix("# branch.ab ") {
            for count in divergence.split_whitespace() {
                if let Some(value) = count.strip_prefix('+') {
                    ahead = value.parse().unwrap_or(0);
                } else if let Some(value) = count.strip_prefix('-') {
                    behind = value.parse().unwrap_or(0);
                }
            }
        } else if line.starts_with("u ") {
            status = WorktreeStatus::Conflicts;
        } else if !line.is_empty() && !line.starts_with("# ") && status != WorktreeStatus::Conflicts
        {
            status = WorktreeStatus::Dirty;
        }
    }

    Some(GitPreviewData {
        branch: branch?,
        status: match status {
            WorktreeStatus::Clean => "✓",
            WorktreeStatus::Dirty => "●",
            WorktreeStatus::Conflicts => "⚠",
        }
        .to_string(),
        ahead,
        behind,
    })
}
