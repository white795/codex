//! App-event routing applies only the current CxLine Git lookup to the live footer.

use super::*;
use crate::workspace_command::WorkspaceCommand;
use crate::workspace_command::WorkspaceCommandError;
use crate::workspace_command::WorkspaceCommandExecutor;
use crate::workspace_command::WorkspaceCommandOutput;
use pretty_assertions::assert_eq;
use std::future::Future;
use std::pin::Pin;

struct FixedGitRunner;

impl WorkspaceCommandExecutor for FixedGitRunner {
    fn run(
        &self,
        _command: WorkspaceCommand,
    ) -> Pin<
        Box<dyn Future<Output = Result<WorkspaceCommandOutput, WorkspaceCommandError>> + Send + '_>,
    > {
        Box::pin(async {
            Ok(WorkspaceCommandOutput {
                exit_code: 0,
                stdout: "# branch.head app-branch\n# branch.ab +2 -0\n? result.txt\n".to_string(),
                stderr: String::new(),
            })
        })
    }
}

#[tokio::test]
async fn cxline_git_app_events_update_the_footer_only_for_the_pending_request() -> Result<()> {
    let (mut app, mut events, _ops) = make_test_app_with_channels().await;
    let mut server = start_config_write_test_app_server(&app).await?;
    let mut tui = crate::tui::test_support::make_test_tui()?;
    let root = app.local_settings.codex_home.join("cxline");
    std::fs::create_dir(&root)?;
    std::fs::write(
        root.join("config.toml"),
        toml::to_string_pretty(&crate::statusline::CxLineConfig::default())?,
    )?;
    app.workspace_command_runner = Some(Arc::new(FixedGitRunner));
    let init = app.chatwidget_init_for_forked_or_resumed_thread(
        &mut tui,
        app.config.clone(),
        /*initial_user_message*/ None,
    );
    app.replace_chat_widget(ChatWidget::new_with_app_event(init));
    app.refresh_status_line();

    let (request_id, cwd, preview) = loop {
        let event = tokio::time::timeout(Duration::from_secs(1), events.recv())
            .await?
            .expect("CxLine event channel remains open");
        if let AppEvent::CxLineGitPreviewUpdated {
            request_id,
            cwd,
            preview,
        } = event
        {
            break (request_id, cwd, preview);
        }
    };
    let initial = app.chat_widget.status_line_text();
    app.handle_event(
        &mut tui,
        &mut server,
        AppEvent::CxLineGitPreviewUpdated {
            request_id: uuid::Uuid::nil(),
            cwd: cwd.clone(),
            preview: preview.clone(),
        },
    )
    .await?;
    assert_eq!(app.chat_widget.status_line_text(), initial);

    app.handle_event(
        &mut tui,
        &mut server,
        AppEvent::CxLineGitPreviewUpdated {
            request_id,
            cwd: cwd.clone(),
            preview,
        },
    )
    .await?;
    let updated = app.chat_widget.status_line_text();
    assert!(
        updated
            .as_ref()
            .expect("live Git preview")
            .contains("app-branch ● ↑2")
    );

    app.handle_event(
        &mut tui,
        &mut server,
        AppEvent::CxLineGitPreviewUpdated {
            request_id,
            cwd,
            preview: None,
        },
    )
    .await?;
    assert_eq!(app.chat_widget.status_line_text(), updated);
    server.shutdown().await?;
    Ok(())
}
