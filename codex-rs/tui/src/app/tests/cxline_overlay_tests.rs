//! App-level CxLine overlay routing.

use super::*;

#[tokio::test]
async fn cxline_escape_closes_the_editor_without_starting_backtrack_preview() -> Result<()> {
    let mut app = make_test_app().await;
    app.transcript_cells = vec![Arc::new(UserHistoryCell {
        spoken: false,
        message: "previous prompt".to_string(),
        text_elements: Vec::new(),
        local_image_paths: Vec::new(),
        remote_image_urls: Vec::new(),
    })];
    let mut server = start_config_write_test_app_server(&app).await?;
    let mut tui = crate::tui::test_support::make_test_tui()?;
    tui.set_alt_screen_enabled(/*enabled*/ false);

    app.handle_event(&mut tui, &mut server, AppEvent::OpenCxlineConfig)
        .await?;
    assert!(matches!(app.overlay, Some(Overlay::Cxline(_))));

    app.handle_backtrack_overlay_event(
        &mut tui,
        &mut server,
        TuiEvent::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
    )
    .await?;

    assert!(app.overlay.is_none());
    assert!(!app.backtrack.overlay_preview_active);
    server.shutdown().await?;
    Ok(())
}
