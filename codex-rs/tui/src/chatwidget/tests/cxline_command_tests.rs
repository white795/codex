//! CxLine slash-command routing.

use super::*;

#[tokio::test]
async fn slash_cxline_requests_the_full_screen_configuration_page() {
    let (mut chat, mut events, _ops) = make_chatwidget_manual(/*model_override*/ None).await;

    chat.dispatch_command(SlashCommand::Cxline);

    assert_matches!(events.try_recv(), Ok(AppEvent::OpenCxlineConfig));
}
