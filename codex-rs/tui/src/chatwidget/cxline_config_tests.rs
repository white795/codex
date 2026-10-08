//! Saved-config opt-in and editor exit baselines, without live footer rendering.

use super::CxLineRuntime;
use crate::statusline::CxLineConfig;
use pretty_assertions::assert_eq;
use std::path::Path;
use tempfile::tempdir;

fn save_config(codex_home: &Path, config: &CxLineConfig) {
    let root = codex_home.join("cxline");
    std::fs::create_dir_all(&root).expect("CxLine directory");
    std::fs::write(
        root.join("config.toml"),
        toml::to_string_pretty(config).expect("serialize CxLine configuration"),
    )
    .expect("save CxLine configuration");
}

#[test]
fn missing_main_config_keeps_the_default_editor_without_initializing_storage() {
    let home = tempdir().expect("temporary CODEX_HOME");

    let runtime = CxLineRuntime::load(home.path());

    assert_eq!(runtime.editor_config(), CxLineConfig::default());
    assert!(!runtime.has_saved_config);
    assert!(!home.path().join("cxline").exists());
}

#[test]
fn saved_main_config_is_loaded_with_its_disabled_setting_intact() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let saved = CxLineConfig {
        enabled: false,
        separator: " / ".to_string(),
        ..CxLineConfig::default()
    };
    save_config(home.path(), &saved);

    let runtime = CxLineRuntime::load(home.path());

    assert_eq!(runtime.editor_config(), saved);
    assert!(runtime.has_saved_config);
}

#[test]
fn malformed_main_config_does_not_opt_in_or_repair_the_file() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let root = home.path().join("cxline");
    std::fs::create_dir(&root).expect("CxLine directory");
    let content = "enabled = [not toml";
    std::fs::write(root.join("config.toml"), content).expect("malformed configuration");

    let runtime = CxLineRuntime::load(home.path());

    assert_eq!(runtime.editor_config(), CxLineConfig::default());
    assert!(!runtime.has_saved_config);
    assert_eq!(
        std::fs::read_to_string(root.join("config.toml")).expect("unmodified configuration"),
        content
    );
}

#[test]
fn editor_theme_selection_without_a_saved_config_does_not_opt_in() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let mut runtime = CxLineRuntime::load(home.path());
    let original = runtime.editor_config();
    let mut selected = original.clone();
    selected.theme = "default".to_string();

    runtime.apply_editor_config(selected, home.path());

    assert_eq!(runtime.editor_config(), original);
    assert!(!runtime.has_saved_config);
    assert!(!home.path().join("cxline").exists());
}

#[test]
fn explicit_save_makes_the_editor_exit_baseline_available_on_reopen() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let mut runtime = CxLineRuntime::load(home.path());
    let saved = CxLineConfig {
        separator: " / ".to_string(),
        ..CxLineConfig::default()
    };
    save_config(home.path(), &saved);

    runtime.apply_editor_config(saved.clone(), home.path());

    assert_eq!(runtime.editor_config(), saved);
    assert!(runtime.has_saved_config);
    assert_eq!(CxLineRuntime::load(home.path()).editor_config(), saved);
}

#[test]
fn existing_saved_config_allows_session_theme_switching_without_rewriting_storage() {
    let home = tempdir().expect("temporary CODEX_HOME");
    let saved = CxLineConfig::default();
    save_config(home.path(), &saved);
    let mut runtime = CxLineRuntime::load(home.path());
    let mut selected = saved.clone();
    selected.theme = "default".to_string();

    runtime.apply_editor_config(selected.clone(), home.path());

    assert_eq!(runtime.editor_config(), selected);
    assert_eq!(CxLineConfig::load_saved(home.path()).unwrap(), Some(saved));
}
