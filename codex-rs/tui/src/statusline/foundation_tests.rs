use super::StatusLineContext;
use super::build_statusline;
use super::config::CxLineConfig;
use super::style::AnsiColor;
use super::style::StyleMode;
use super::themes::ThemePresets;
use codex_protocol::openai_models::ReasoningEffort;
use pretty_assertions::assert_eq;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

const LEGACY_CONFIG: &str = r#"
enabled = false
theme = "my-theme"
style = "powerline"
separator = " / "

[segments.model]
id = "model"
enabled = true
icon = { plain = "模型", nerd_font = "M" }
colors = { icon = { c16 = 6 }, text = { c256 = 208 }, background = { r = 12, g = 34, b = 56 } }
styles = { text_bold = true }
options = { custom_label = "主模型", count = 3, compact = true }

[segments.git]
id = "git"
enabled = false
"#;

#[test]
fn legacy_configuration_round_trips_custom_colors_icons_and_options() {
    let config: CxLineConfig = toml::from_str(LEGACY_CONFIG).unwrap();
    assert!(!config.enabled);
    assert_eq!(config.theme, "my-theme");
    assert_eq!(config.style, StyleMode::Powerline);
    assert_eq!(config.separator, " / ");
    assert_eq!(config.segments.model.colors.icon, Some(AnsiColor::c16(6)));
    assert_eq!(
        config.segments.model.colors.text,
        Some(AnsiColor::c256(208))
    );
    assert_eq!(
        config.segments.model.colors.background,
        Some(AnsiColor::rgb(12, 34, 56))
    );
    assert_eq!(config.segments.model.icon.plain, "模型");
    assert_eq!(
        serde_json::to_value(&config.segments.model.options).unwrap(),
        serde_json::json!({ "custom_label": "主模型", "count": 3, "compact": true })
    );
    assert!(!config.segments.git.enabled);
    let serialized = toml::to_string_pretty(&config).unwrap();
    let restored: CxLineConfig = toml::from_str(&serialized).unwrap();
    assert_eq!(restored, config);
}

#[test]
fn omitted_fields_keep_legacy_serde_defaults() {
    // Legacy empty TOML uses the default theme's segments, unlike a missing file,
    // which uses the Cometix theme. Preserve that distinction during migration.
    let config: CxLineConfig = toml::from_str("").unwrap();
    let expected = CxLineConfig {
        segments: ThemePresets::get_default().segments,
        ..CxLineConfig::default()
    };
    assert_eq!(config, expected);
}

#[test]
fn all_builtin_themes_round_trip_through_the_legacy_toml_format() {
    for name in [
        "default",
        "cometix",
        "minimal",
        "gruvbox",
        "nord",
        "powerline-dark",
        "powerline-light",
        "powerline-rose-pine",
        "powerline-tokyo-night",
    ] {
        let theme = ThemePresets::get_builtin(name).unwrap();
        let serialized = toml::to_string_pretty(&theme).unwrap();
        assert_eq!(toml::from_str::<CxLineConfig>(&serialized).unwrap(), theme);
    }
}

#[test]
fn zero_context_window_and_weekly_only_usage_render_without_division_or_missing_usage() {
    let mut config = ThemePresets::get_default();
    config.segments.model.enabled = false;
    config.segments.directory.enabled = false;
    config.segments.git.enabled = false;
    let ctx = StatusLineContext::new("", Path::new(""))
        .with_context(Some(1500), Some(0))
        .with_rate_limit(
            /*hourly_percent*/ None,
            Some(50.0),
            /*weekly_resets_at*/ None,
        );
    let line = build_statusline(&config, &ctx).render_line();
    assert_eq!(line.to_string(), "⚡️ 1.5k tokens │ \u{f0aa1} 50%");
}

#[test]
fn loading_uses_the_supplied_codex_home_without_changing_the_file() {
    let home = tempfile::tempdir().unwrap();
    let cxline = home.path().join("cxline");
    fs::create_dir(&cxline).unwrap();
    let path = cxline.join("config.toml");
    fs::write(&path, LEGACY_CONFIG).unwrap();
    assert_eq!(
        CxLineConfig::load(home.path()).unwrap(),
        toml::from_str(LEGACY_CONFIG).unwrap()
    );
    assert_eq!(fs::read_to_string(path).unwrap(), LEGACY_CONFIG);
    assert!(!cxline.join("themes").exists());
}

#[test]
fn missing_configuration_returns_defaults_without_creating_directories() {
    let root = tempfile::tempdir().unwrap();
    let home = root.path().join("candidate-home");
    assert_eq!(CxLineConfig::load(&home).unwrap(), CxLineConfig::default());
    assert!(!home.exists());
}

#[test]
fn malformed_configuration_is_reported_without_overwriting_it() {
    let home = tempfile::tempdir().unwrap();
    fs::create_dir(home.path().join("cxline")).unwrap();
    let path = home.path().join("cxline/config.toml");
    fs::write(&path, "enabled = [").unwrap();
    assert_eq!(
        CxLineConfig::load(home.path()).unwrap_err().kind(),
        ErrorKind::InvalidData
    );
    assert_eq!(fs::read_to_string(path).unwrap(), "enabled = [");
}

#[test]
fn unreadable_configuration_is_not_treated_as_a_missing_file() {
    let home = tempfile::tempdir().unwrap();
    fs::create_dir_all(home.path().join("cxline/config.toml")).unwrap();
    assert!(CxLineConfig::load(home.path()).is_err());
}

#[test]
fn fixed_runtime_data_renders_plain_style_and_colors() {
    assert_theme_rendering("default");
}

#[test]
fn fixed_runtime_data_renders_nerd_font_style_and_colors() {
    assert_theme_rendering("cometix");
}

#[test]
fn fixed_runtime_data_renders_powerline_style_and_colors() {
    assert_theme_rendering("powerline-dark");
}

fn assert_theme_rendering(theme: &str) {
    let ctx = StatusLineContext::new("gpt-5.2-codex", Path::new("/workspace/codex"))
        .with_reasoning_effort(Some(ReasoningEffort::Medium))
        .with_context(Some(50_000), Some(128_000))
        .with_rate_limit(Some(25.0), Some(15.0), Some("1-28-14".to_string()))
        .with_git_preview("main", "✓", /*ahead*/ 2, /*behind*/ 1);
    let config = ThemePresets::get_builtin(theme).unwrap();
    let line = build_statusline(&config, &ctx).render_line();
    let mut buffers = Vec::new();
    for width in [120, 24, 1, 0] {
        let area = Rect::new(/*x*/ 0, /*y*/ 0, width, /*height*/ 1);
        let mut buffer = Buffer::empty(area);
        buffer.set_line(/*x*/ 0, /*y*/ 0, &line, width);
        buffers.push(buffer);
    }
    insta::assert_debug_snapshot!(format!("cxline_{theme}"), (line, buffers));
}

#[test]
fn unavailable_data_keeps_context_placeholder_and_omits_other_segments() {
    let config = ThemePresets::get_default();
    let ctx = StatusLineContext::new("", Path::new(""));
    let line = build_statusline(&config, &ctx).render_line();
    insta::assert_debug_snapshot!("cxline_missing_data", line);
}

#[test]
fn disabling_all_segments_renders_an_empty_line() {
    let mut config = CxLineConfig::default();
    config.segments.model.enabled = false;
    config.segments.directory.enabled = false;
    config.segments.git.enabled = false;
    config.segments.context.enabled = false;
    config.segments.usage.enabled = false;
    let ctx = StatusLineContext::new("gpt-5.2-codex", Path::new("/workspace/codex"));
    assert_eq!(
        build_statusline(&config, &ctx).render_line(),
        ratatui::text::Line::default()
    );
}

#[test]
fn persistent_and_custom_reasoning_efforts_remain_visible() {
    let mut config = ThemePresets::get_default();
    config.segments.directory.enabled = false;
    config.segments.git.enabled = false;
    config.segments.context.enabled = false;
    config.segments.usage.enabled = false;
    for (effort, expected) in [
        (ReasoningEffort::Persistent, "🤖 custom-model · persistent"),
        (
            ReasoningEffort::Custom("future".to_string()),
            "🤖 custom-model future",
        ),
    ] {
        let ctx = StatusLineContext::new("custom-model", Path::new(""))
            .with_reasoning_effort(Some(effort));
        assert_eq!(
            build_statusline(&config, &ctx).render_line().to_string(),
            expected
        );
    }
}
