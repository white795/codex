//! CxLine full-screen overlay lifecycle.

use super::CxLineOverlay;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use ratatui::layout::Size;
use tempfile::tempdir;

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn cxline_missing_config_loads_the_default_without_initializing_storage() {
    let home = tempdir().expect("temporary CODEX_HOME");

    let overlay = CxLineOverlay::new(home.path(), super::config::CxLineConfig::default());

    assert_eq!(overlay.draft().theme, "cometix");
    assert!(!home.path().join("cxline").exists());
}

#[test]
fn cxline_malformed_config_reports_the_error_and_keeps_the_page_available() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let root = home.path().join("cxline");
    std::fs::create_dir(&root).expect("cxline directory");
    std::fs::write(root.join("config.toml"), "enabled = [not toml").expect("malformed config");

    let overlay = CxLineOverlay::new(home.path(), super::config::CxLineConfig::default());

    assert_eq!(overlay.draft().theme, "cometix");
    assert!(
        overlay
            .status_message()
            .is_some_and(|message| message.starts_with("Failed to load configuration:"))
    );
}

#[test]
fn cxline_save_key_persists_the_loaded_draft() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let mut overlay = CxLineOverlay::new(home.path(), super::config::CxLineConfig::default());

    overlay.handle_key_event(key(KeyCode::Char('s')));

    let saved = std::fs::read_to_string(home.path().join("cxline/config.toml"))
        .expect("saved configuration");
    assert!(saved.contains("theme = \"cometix\""));
    assert_eq!(overlay.status_message(), Some("Configuration saved!"));
}

#[test]
fn cxline_master_switch_saves_and_reopens_without_losing_customization() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let original = super::config::CxLineConfig {
        separator: " / ".to_string(),
        ..super::themes::ThemePresets::get_nord()
    };
    let mut overlay = CxLineOverlay::new(home.path(), original.clone());

    overlay.handle_key_event(key(KeyCode::Char('t')));
    overlay.handle_key_event(key(KeyCode::Char('s')));
    overlay.handle_key_event(key(KeyCode::Esc));

    let disabled = super::config::CxLineConfig {
        enabled: false,
        ..original.clone()
    };
    assert!(overlay.is_done());
    assert_eq!(overlay.config_for_exit(), disabled);
    assert_eq!(
        super::config::CxLineConfig::load_saved(home.path()).unwrap(),
        Some(disabled.clone())
    );

    let mut reopened = CxLineOverlay::new(home.path(), disabled);
    reopened.handle_key_event(key(KeyCode::Char('T')));
    reopened.handle_key_event(key(KeyCode::Char('s')));
    reopened.handle_key_event(key(KeyCode::Esc));

    assert!(reopened.is_done());
    assert_eq!(reopened.config_for_exit(), original);
    assert_eq!(
        super::config::CxLineConfig::load_saved(home.path()).unwrap(),
        Some(original)
    );
}

#[test]
fn cxline_unsaved_master_switch_does_not_change_the_exit_config_or_storage() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let original = super::config::CxLineConfig::default();
    let mut overlay = CxLineOverlay::new(home.path(), original.clone());

    overlay.handle_key_event(key(KeyCode::Char('t')));
    overlay.handle_key_event(key(KeyCode::Esc));

    assert!(!overlay.draft().enabled);
    assert!(overlay.is_done());
    assert_eq!(overlay.config_for_exit(), original);
    assert!(!home.path().join("cxline").exists());
}

#[test]
fn cxline_child_dialog_consumes_escape_before_the_page_exits() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let mut overlay = CxLineOverlay::new(home.path(), super::config::CxLineConfig::default());

    overlay.handle_key_event(key(KeyCode::Char('e')));
    overlay.handle_key_event(key(KeyCode::Esc));
    assert!(!overlay.is_done());

    overlay.handle_key_event(key(KeyCode::Esc));
    assert!(overlay.is_done());
}

#[tokio::test]
async fn cxline_viewport_events_redraw_without_closing_the_page() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let mut overlay = CxLineOverlay::new(home.path(), super::config::CxLineConfig::default());
    let mut tui = crate::tui::test_support::make_test_tui().expect("test TUI");
    tui.set_alt_screen_enabled(/*enabled*/ false);

    for event in [
        crate::tui::TuiEvent::Draw,
        crate::tui::TuiEvent::Resize(Size::new(72, 20)),
        crate::tui::TuiEvent::Resume,
        crate::tui::TuiEvent::FocusGained,
    ] {
        overlay
            .handle_event(&mut tui, event)
            .expect("viewport redraw");
        assert!(!overlay.is_done());
    }
}
