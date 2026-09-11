use super::config::CxLineConfig;
use super::storage::CxLineStore;
use super::themes::ThemePresets;
use pretty_assertions::assert_eq;
use std::fs;
use std::io::ErrorKind;

#[test]
fn missing_config_uses_the_existing_cometix_theme_without_creating_config() {
    let home = tempfile::tempdir().unwrap();
    let themes = home.path().join("cxline/themes");
    fs::create_dir_all(&themes).unwrap();
    let customized = CxLineConfig {
        separator: " / ".to_string(),
        ..CxLineConfig::default()
    };
    let content = toml::to_string_pretty(&customized).unwrap();
    fs::write(themes.join("cometix.toml"), &content).unwrap();

    assert_eq!(CxLineConfig::load(home.path()).unwrap(), customized);
    assert!(!home.path().join("cxline/config.toml").exists());
    assert_eq!(
        fs::read_to_string(themes.join("cometix.toml")).unwrap(),
        content
    );
}

#[test]
fn saving_config_replaces_only_the_selected_homes_config() {
    let home = tempfile::tempdir().unwrap();
    let other = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let original = CxLineConfig::default();
    CxLineStore::new(other.path())
        .save_config(&original)
        .unwrap();
    store.save_config(&original).unwrap();
    let edited = CxLineConfig {
        enabled: false,
        separator: " / ".into(),
        ..original.clone()
    };
    store.save_config(&edited).unwrap();
    assert_eq!(CxLineConfig::load(home.path()).unwrap(), edited);
    assert_eq!(CxLineConfig::load(other.path()).unwrap(), original);
    assert!(!home.path().join("cxline/themes").exists());
}

#[test]
fn existing_config_keeps_its_edits_instead_of_reapplying_the_theme() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let original = CxLineConfig::default();
    store.save_config(&original).unwrap();
    let theme = CxLineConfig {
        separator: " changed theme ".into(),
        ..original.clone()
    };
    store.save_theme(&theme).unwrap();
    assert_eq!(store.load_theme("cometix").unwrap(), theme);
    assert_eq!(CxLineConfig::load(home.path()).unwrap(), original);
}

#[test]
fn saving_a_new_unicode_theme_preserves_settings_and_does_not_mutate_the_draft() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let draft = ThemePresets::get_builtin("nord").unwrap();
    store.save_new_theme("我的 深色主题", &draft).unwrap();
    let expected = CxLineConfig {
        theme: "我的 深色主题".into(),
        ..draft.clone()
    };
    assert_eq!(store.load_theme("我的 深色主题").unwrap(), expected);
    assert_eq!(draft, ThemePresets::get_builtin("nord").unwrap());
    assert!(!home.path().join("cxline/config.toml").exists());
}

#[test]
fn save_as_rejects_an_existing_name_but_writing_the_current_theme_replaces_it() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    store
        .save_new_theme("custom", &CxLineConfig::default())
        .unwrap();
    let path = home.path().join("cxline/themes/custom.toml");
    let before = fs::read(&path).unwrap();
    let edited = CxLineConfig {
        theme: "custom".into(),
        separator: " / ".into(),
        ..CxLineConfig::default()
    };
    assert_eq!(
        store.save_new_theme("custom", &edited).unwrap_err().kind(),
        ErrorKind::AlreadyExists
    );
    assert_eq!(fs::read(&path).unwrap(), before);
    store.save_theme(&edited).unwrap();
    assert_eq!(store.load_theme("custom").unwrap(), edited);
    assert_eq!(fs::read_dir(path.parent().unwrap()).unwrap().count(), 1);
}

#[test]
fn invalid_theme_names_are_rejected_before_any_directory_is_created() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    for name in [
        "",
        ".",
        "..",
        "../outside",
        "a/b",
        r"a\b",
        "/tmp/theme",
        "C:theme",
        "x\n",
        "a*",
        "a?",
        "a|",
        "a<",
        "a>",
        "a\"",
        "a\0",
        "trailing.",
        "trailing ",
        "CON",
        "nul",
        "COM1",
        "lpt9",
        "con.backup",
        "LPT¹",
        "CONIN$",
        "CONOUT$",
    ] {
        let config = CxLineConfig {
            theme: name.into(),
            ..CxLineConfig::default()
        };
        assert_eq!(
            store.load_theme(name).unwrap_err().kind(),
            ErrorKind::InvalidInput,
            "{name:?}"
        );
        assert_eq!(
            store.save_new_theme(name, &config).unwrap_err().kind(),
            ErrorKind::InvalidInput,
            "{name:?}"
        );
        assert_eq!(
            store.save_theme(&config).unwrap_err().kind(),
            ErrorKind::InvalidInput,
            "{name:?}"
        );
    }
    assert!(
        store
            .save_new_theme(&"中".repeat(90), &CxLineConfig::default())
            .is_err()
    );
    assert_eq!(fs::read_dir(home.path()).unwrap().count(), 0);
}

#[test]
fn missing_theme_files_use_builtins_without_materializing_them() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    assert_eq!(
        store.load_theme("nord").unwrap(),
        ThemePresets::get_builtin("nord").unwrap()
    );
    assert_eq!(
        store.load_theme("unknown").unwrap(),
        ThemePresets::get_default()
    );
    assert!(!home.path().join("cxline").exists());
}

#[test]
fn malformed_theme_files_fall_back_without_overwriting_the_original() {
    let home = tempfile::tempdir().unwrap();
    let themes = home.path().join("cxline/themes");
    fs::create_dir_all(&themes).unwrap();
    let store = CxLineStore::new(home.path());
    for (name, expected) in [
        ("cometix", CxLineConfig::default()),
        ("custom", ThemePresets::get_default()),
    ] {
        let path = themes.join(format!("{name}.toml"));
        fs::write(&path, "style = [").unwrap();
        assert_eq!(store.load_theme(name).unwrap(), expected);
        assert_eq!(fs::read_to_string(path).unwrap(), "style = [");
    }
    assert_eq!(
        CxLineConfig::load(home.path()).unwrap(),
        CxLineConfig::default()
    );
}

#[test]
fn serialization_failure_preserves_the_previous_config_and_leaves_no_temporary_file() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let mut config = CxLineConfig::default();
    store.save_config(&config).unwrap();
    let path = home.path().join("cxline/config.toml");
    let before = fs::read(&path).unwrap();
    config
        .segments
        .model
        .options
        .insert("unsupported".into(), serde_json::Value::Null);
    assert_eq!(
        store.save_config(&config).unwrap_err().kind(),
        ErrorKind::InvalidData
    );
    assert_eq!(fs::read(path).unwrap(), before);
    assert_eq!(fs::read_dir(home.path().join("cxline")).unwrap().count(), 1);
}

#[test]
fn failed_destination_replacement_preserves_the_destination_and_cleans_temporary_files() {
    let home = tempfile::tempdir().unwrap();
    let path = home.path().join("cxline/config.toml");
    fs::create_dir_all(&path).unwrap();
    assert!(
        CxLineStore::new(home.path())
            .save_config(&CxLineConfig::default())
            .is_err()
    );
    assert!(path.is_dir());
    assert_eq!(fs::read_dir(path.parent().unwrap()).unwrap().count(), 1);
}

#[test]
fn concurrent_save_as_operations_create_exactly_one_complete_theme() {
    let home = tempfile::tempdir().unwrap();
    let store = CxLineStore::new(home.path());
    let barrier = std::sync::Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let handles = ["first", "second"].map(|separator| {
            let store = &store;
            let barrier = &barrier;
            scope.spawn(move || {
                let config = CxLineConfig {
                    theme: "race".into(),
                    separator: separator.into(),
                    ..CxLineConfig::default()
                };
                barrier.wait();
                let result = store.save_new_theme("race", &config);
                (config, result)
            })
        });
        handles.map(|handle| handle.join().unwrap())
    });
    assert_eq!(
        results.iter().filter(|(_, result)| result.is_ok()).count(),
        1
    );
    for (config, result) in &results {
        match result {
            Ok(()) => assert_eq!(store.load_theme("race").unwrap(), *config),
            Err(error) => assert_eq!(error.kind(), ErrorKind::AlreadyExists),
        }
    }
    assert_eq!(
        fs::read_dir(home.path().join("cxline/themes"))
            .unwrap()
            .count(),
        1
    );
}

#[cfg(unix)]
#[test]
fn saving_does_not_replace_symlink_destinations_or_modify_their_targets() {
    let home = tempfile::tempdir().unwrap();
    let outside = home.path().join("outside.toml");
    fs::write(&outside, "keep me").unwrap();
    let directory = home.path().join("cxline/themes");
    fs::create_dir_all(&directory).unwrap();
    let path = directory.join("cometix.toml");
    std::os::unix::fs::symlink(&outside, &path).unwrap();
    let store = CxLineStore::new(home.path());
    assert!(store.save_theme(&CxLineConfig::default()).is_err());
    assert_eq!(
        store
            .save_new_theme("cometix", &CxLineConfig::default())
            .unwrap_err()
            .kind(),
        ErrorKind::AlreadyExists
    );
    assert!(fs::symlink_metadata(path).unwrap().file_type().is_symlink());
    assert_eq!(fs::read_to_string(outside).unwrap(), "keep me");
}
