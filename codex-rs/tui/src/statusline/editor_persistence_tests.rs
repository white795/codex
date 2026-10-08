use super::CxLineEditor;
use super::EditorCommand;
use super::EditorOutcome;
use crate::statusline::config::CxLineConfig;
use crate::statusline::storage::CxLineStore;
use crate::statusline::themes::ThemePresets;
use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn selecting_and_resetting_themes_uses_the_supplied_store_and_preserves_enabled() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let themes = home.path().join("cxline/themes");
    fs::create_dir_all(&themes).unwrap();

    let mut stored_nord = ThemePresets::get_nord();
    stored_nord.theme = "mismatched-file-value".to_string();
    stored_nord.separator = " stored nord ".to_string();
    fs::write(
        themes.join("nord.toml"),
        toml::to_string_pretty(&stored_nord).unwrap(),
    )
    .unwrap();
    let stored_cometix = CxLineConfig {
        separator: " stored reset ".to_string(),
        ..ThemePresets::get_cometix()
    };
    store.save_theme(&stored_cometix).unwrap();

    let mut original = ThemePresets::get_cometix();
    original.enabled = false;
    let mut editor = CxLineEditor::new(original);
    assert_eq!(
        editor.execute_command(&store, EditorCommand::SelectTheme("nord")),
        EditorOutcome::Continue
    );
    let expected_nord = CxLineConfig {
        enabled: false,
        theme: "nord".to_string(),
        ..stored_nord
    };
    assert_eq!(editor.draft, expected_nord);
    assert_eq!(editor.status_message.as_deref(), Some("Theme: nord"));

    editor.draft.separator = " unsaved ".to_string();
    assert_eq!(
        editor.execute_command(&store, EditorCommand::ResetTheme("cometix".to_string())),
        EditorOutcome::Continue
    );
    let expected_reset = CxLineConfig {
        enabled: false,
        ..stored_cometix
    };
    assert_eq!(editor.draft, expected_reset);
    assert_eq!(editor.status_message.as_deref(), Some("Reset to: cometix"));
}

#[test]
fn saving_config_persists_the_draft_and_updates_the_exit_baseline() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    editor.draft.separator = " saved config ".to_string();
    editor.draft.segments.context.enabled = false;
    let expected = editor.draft.clone();

    assert_eq!(
        editor.execute_command(&store, EditorCommand::SaveConfig),
        EditorOutcome::Continue
    );

    assert_eq!(store.load_config().unwrap(), expected);
    assert_eq!(editor.config_for_exit(), expected);
    assert_eq!(
        editor.status_message.as_deref(),
        Some("Configuration saved!")
    );
}

#[test]
fn a_failed_config_save_keeps_the_draft_and_previous_exit_baseline() {
    let home = tempfile::tempdir().unwrap();
    fs::write(home.path().join("cxline"), "not a directory").unwrap();
    let store = CxLineStore::new(home.path());
    let original = ThemePresets::get_cometix();
    let mut editor = CxLineEditor::new(original.clone());
    editor.draft.separator = " unsaved ".to_string();
    let draft = editor.draft.clone();

    assert_eq!(
        editor.execute_command(&store, EditorCommand::SaveConfig),
        EditorOutcome::Continue
    );

    assert_eq!(editor.draft, draft);
    assert_eq!(editor.config_for_exit(), original);
    assert!(
        editor
            .status_message
            .as_deref()
            .is_some_and(|message| message.starts_with("Failed to save: "))
    );
    assert_eq!(
        fs::read_to_string(home.path().join("cxline")).unwrap(),
        "not a directory"
    );
}

#[test]
fn writing_the_current_theme_does_not_mark_the_main_config_as_saved() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let original = ThemePresets::get_cometix();
    let mut editor = CxLineEditor::new(original.clone());
    editor.draft.separator = " theme only ".to_string();
    let written = editor.draft.clone();

    assert_eq!(
        editor.execute_command(&store, EditorCommand::SaveTheme),
        EditorOutcome::Continue
    );

    assert_eq!(store.load_theme("cometix").unwrap(), written);
    assert_eq!(editor.config_for_exit(), original);
    assert_eq!(
        editor.status_message.as_deref(),
        Some("Wrote config to theme: cometix")
    );
    assert!(!home.path().join("cxline/config.toml").exists());
}

#[test]
fn saving_a_new_theme_updates_its_exit_baseline_but_never_overwrites_it() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    editor.draft.separator = " new theme ".to_string();

    assert_eq!(
        editor.execute_command(&store, EditorCommand::SaveNewTheme("我的主题".to_string())),
        EditorOutcome::Continue
    );
    let saved = editor.draft.clone();
    assert_eq!(saved.theme, "我的主题");
    assert_eq!(store.load_theme("我的主题").unwrap(), saved);
    assert_eq!(editor.config_for_exit(), saved);
    assert_eq!(
        editor.status_message.as_deref(),
        Some("Saved as new theme: 我的主题")
    );

    editor.draft.separator = " rejected replacement ".to_string();
    let draft = editor.draft.clone();
    assert_eq!(
        editor.execute_command(&store, EditorCommand::SaveNewTheme("我的主题".to_string())),
        EditorOutcome::Continue
    );
    assert_eq!(editor.draft, draft);
    assert_eq!(store.load_theme("我的主题").unwrap(), saved);
    assert_eq!(editor.config_for_exit(), saved);
    assert!(
        editor
            .status_message
            .as_deref()
            .is_some_and(|message| message.starts_with("Failed to save theme: "))
    );
    assert!(!home.path().join("cxline/config.toml").exists());
}

#[test]
fn exit_returns_the_legacy_exit_config_without_touching_storage() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let original = ThemePresets::get_cometix();
    let mut editor = CxLineEditor::new(original.clone());
    editor.draft.separator = " unsaved ".to_string();

    assert_eq!(
        editor.execute_command(&store, EditorCommand::None),
        EditorOutcome::Continue
    );
    assert_eq!(
        editor.execute_command(&store, EditorCommand::Exit),
        EditorOutcome::Exit
    );
    assert_eq!(editor.config_for_exit(), original);
    assert!(!home.path().join("cxline").exists());
}
