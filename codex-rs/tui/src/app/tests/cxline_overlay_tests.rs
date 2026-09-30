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

#[tokio::test]
async fn saving_and_closing_cxline_applies_it_to_the_live_footer() -> Result<()> {
    let mut app = make_test_app().await;
    let official_line = app.chat_widget.status_line_text();
    let mut server = start_config_write_test_app_server(&app).await?;
    let mut tui = crate::tui::test_support::make_test_tui()?;
    tui.set_alt_screen_enabled(/*enabled*/ false);

    app.handle_event(&mut tui, &mut server, AppEvent::OpenCxlineConfig)
        .await?;
    app.handle_backtrack_overlay_event(
        &mut tui,
        &mut server,
        TuiEvent::Key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)),
    )
    .await?;
    app.handle_backtrack_overlay_event(
        &mut tui,
        &mut server,
        TuiEvent::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
    )
    .await?;

    let cxline = app.chat_widget.status_line_text().expect("CxLine footer");
    assert_ne!(Some(cxline.clone()), official_line);
    assert!(cxline.contains("- · - tokens"));
    server.shutdown().await?;
    Ok(())
}
