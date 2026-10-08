//! App-level CxLine editor routing, persistence, and draft restoration.

use super::*;
use pretty_assertions::assert_eq;

async fn press_cxline_key(
    app: &mut App,
    tui: &mut crate::tui::Tui,
    server: &mut AppServerSession,
    code: KeyCode,
) -> Result<()> {
    app.handle_tui_event(
        tui,
        server,
        TuiEvent::Key(KeyEvent::new(code, KeyModifiers::NONE)),
    )
    .await?;
    Ok(())
}

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

    press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Esc).await?;

    assert!(app.overlay.is_none());
    assert!(!app.backtrack.overlay_preview_active);
    assert!(!app.local_settings.codex_home.join("cxline").exists());
    server.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn saving_and_closing_cxline_retains_the_configuration_for_reopen() -> Result<()> {
    let mut app = make_test_app().await;
    let official_line = app.chat_widget.status_line_text();
    let mut server = start_config_write_test_app_server(&app).await?;
    let mut tui = crate::tui::test_support::make_test_tui()?;
    tui.set_alt_screen_enabled(/*enabled*/ false);

    app.handle_event(&mut tui, &mut server, AppEvent::OpenCxlineConfig)
        .await?;
    press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Enter).await?;
    press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Char('s')).await?;
    press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Esc).await?;

    let config = app.chat_widget.cxline_editor_config();
    assert!(!config.segments.model.enabled);
    assert_eq!(
        crate::statusline::CxLineConfig::load_saved(&app.local_settings.codex_home)?,
        Some(config.clone())
    );
    assert!(app.overlay.is_none());

    let cxline = app
        .chat_widget
        .status_line_text()
        .expect("saved CxLine footer");
    assert_ne!(Some(cxline.clone()), official_line);
    assert!(cxline.contains("- · - tokens"));

    app.handle_event(&mut tui, &mut server, AppEvent::OpenCxlineConfig)
        .await?;
    let overlay = app.overlay.as_ref().expect("reopened CxLine editor");
    assert_eq!(overlay.cxline_config_for_exit(), Some(config));
    press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Esc).await?;
    server.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn cxline_child_dialog_escape_keeps_the_page_open_and_discards_unsaved_edits() -> Result<()> {
    let mut app = make_test_app().await;
    let original = app.chat_widget.cxline_editor_config();
    let mut server = start_config_write_test_app_server(&app).await?;
    let mut tui = crate::tui::test_support::make_test_tui()?;
    tui.set_alt_screen_enabled(/*enabled*/ false);

    app.handle_event(&mut tui, &mut server, AppEvent::OpenCxlineConfig)
        .await?;
    press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Enter).await?;
    press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Char('e')).await?;
    press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Esc).await?;
    assert!(matches!(app.overlay, Some(Overlay::Cxline(_))));
    assert!(!app.backtrack.overlay_preview_active);

    press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Esc).await?;

    assert!(app.overlay.is_none());
    assert_eq!(app.chat_widget.cxline_editor_config(), original);
    assert!(!app.local_settings.codex_home.join("cxline").exists());
    server.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn cxline_close_restores_the_composer_in_inline_and_owned_sessions() -> Result<()> {
    let mut app = make_test_app().await;
    let mut server = start_config_write_test_app_server(&app).await?;
    let mut tui = crate::tui::test_support::make_test_tui()?;
    app.chat_widget
        .apply_external_edit("preserved draft".into());

    for owned in [false, true] {
        tui.set_owned_screen(owned)?;
        app.handle_event(&mut tui, &mut server, AppEvent::OpenCxlineConfig)
            .await?;
        press_cxline_key(&mut app, &mut tui, &mut server, KeyCode::Esc).await?;

        assert!(app.overlay.is_none());
        assert_eq!(tui.is_owned_screen(), owned);
        assert_eq!(
            app.chat_widget.composer_text_with_pending(),
            "preserved draft"
        );
    }

    tui.set_owned_screen(/*owned*/ false)?;
    server.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn cxline_token_notification_refreshes_the_live_footer_through_app_routing() {
    let mut app = make_test_app().await;
    let thread_id = ThreadId::new();
    app.chat_widget
        .handle_thread_session(test_thread_session(thread_id, app.config.cwd.to_path_buf()));
    let config = crate::statusline::CxLineConfig::default();
    let root = app.local_settings.codex_home.join("cxline");
    std::fs::create_dir(&root).expect("CxLine directory");
    std::fs::write(
        root.join("config.toml"),
        toml::to_string_pretty(&config).expect("serialize CxLine configuration"),
    )
    .expect("saved CxLine configuration");
    app.chat_widget.apply_cxline_editor_config(config);

    app.handle_thread_event_now(ThreadBufferedEvent::Notification(Box::new(
        token_usage_notification(thread_id, "turn-1", Some(100)),
    )));

    let line = app
        .chat_widget
        .status_line_text()
        .expect("live CxLine footer");
    assert!(line.contains("10% · 10 tokens"));
    assert!(!line.contains("- · - tokens"));
}
