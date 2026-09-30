use super::*;
use crate::statusline::CxLineConfig;
use codex_protocol::openai_models::ReasoningEffort;
use pretty_assertions::assert_eq;

fn save_cxline_config(chat: &ChatWidget, config: &CxLineConfig) {
    let root = chat.local_settings.codex_home.join("cxline");
    std::fs::create_dir_all(&root).expect("create CxLine config directory");
    std::fs::write(
        root.join("config.toml"),
        toml::to_string_pretty(config).expect("serialize CxLine config"),
    )
    .expect("save CxLine config");
}

fn paired_usage_snapshot(
    limit_id: &str,
    primary_used_percent: i32,
    secondary_used_percent: i32,
) -> RateLimitSnapshot {
    RateLimitSnapshot {
        limit_id: Some(limit_id.to_string()),
        limit_name: Some(limit_id.to_string()),
        primary: Some(RateLimitWindow {
            used_percent: primary_used_percent,
            window_duration_mins: Some(5 * 60),
            resets_at: None,
        }),
        secondary: Some(RateLimitWindow {
            used_percent: secondary_used_percent,
            window_duration_mins: Some(7 * 24 * 60),
            resets_at: None,
        }),
        ..snapshot(/*percent*/ 0.0)
    }
}

#[tokio::test]
async fn missing_cxline_config_keeps_the_official_status_line() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    chat.local_settings.tui.status_line = Some(vec!["hostname".to_string()]);

    chat.refresh_status_line();

    assert_eq!(status_line_text(&chat), codex_config::os_host_name());
}

#[tokio::test]
async fn saved_cxline_config_renders_live_model_directory_and_context() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.current_cwd = Some(std::path::PathBuf::from("/workspace/cxline-demo"));
    chat.set_reasoning_effort(Some(ReasoningEffort::High));
    chat.set_token_info(Some(make_token_info(
        /*total_tokens*/ 64_000, /*context_window*/ 128_000,
    )));

    chat.apply_cxline_editor_config(config);

    let line = status_line_text(&chat).expect("CxLine footer");
    assert!(line.contains("GPT 5.2 Codex ·high"));
    assert!(line.contains("cxline-demo"));
    assert!(line.contains("50% · 64.0k tokens"));
    insta::assert_snapshot!("cxline_live_base_data", line);
}

#[tokio::test]
async fn disabled_saved_cxline_config_restores_the_official_status_line() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    chat.local_settings.tui.status_line = Some(vec!["hostname".to_string()]);
    let config = CxLineConfig {
        enabled: false,
        ..CxLineConfig::default()
    };
    save_cxline_config(&chat, &config);

    chat.apply_cxline_editor_config(config);

    assert_eq!(status_line_text(&chat), codex_config::os_host_name());
}

#[test]
fn saved_cxline_config_is_available_during_runtime_startup() {
    let home = tempfile::tempdir().expect("temporary CODEX_HOME");
    let root = home.path().join("cxline");
    std::fs::create_dir_all(&root).expect("create CxLine config directory");
    let saved = CxLineConfig::default();
    std::fs::write(
        root.join("config.toml"),
        toml::to_string_pretty(&saved).expect("serialize CxLine config"),
    )
    .expect("save CxLine config");

    let runtime = crate::chatwidget::cxline::CxLineRuntime::load(home.path());

    assert_eq!(runtime.enabled_config(), Some(&saved));
}

#[tokio::test]
async fn cxline_uses_only_codex_rate_limits_and_tracks_window_updates() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.current_cwd = Some(std::path::PathBuf::from("/workspace/cxline-demo"));
    chat.apply_cxline_editor_config(config);

    chat.on_rate_limit_snapshot(Some(paired_usage_snapshot(
        "codex_other",
        /*primary_used_percent*/ 91,
        /*secondary_used_percent*/ 88,
    )));
    assert!(
        !status_line_text(&chat)
            .expect("CxLine footer")
            .contains("91%")
    );

    chat.on_rate_limit_snapshot(Some(paired_usage_snapshot(
        "codex", /*primary_used_percent*/ 25, /*secondary_used_percent*/ 40,
    )));
    let initial = status_line_text(&chat).expect("CxLine footer with usage");
    assert!(initial.contains("25%"));
    insta::assert_snapshot!("cxline_live_rate_limits", initial);

    chat.on_rate_limit_snapshot(Some(paired_usage_snapshot(
        "codex", /*primary_used_percent*/ 63, /*secondary_used_percent*/ 77,
    )));
    let updated = status_line_text(&chat).expect("updated CxLine footer");
    assert!(updated.contains("63%"));
    assert!(!updated.contains("25%"));
}

#[tokio::test]
async fn cxline_weekly_only_usage_uses_the_weekly_percent_and_reset_label() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: Some("codex".to_string()),
        limit_name: Some("codex".to_string()),
        primary: Some(RateLimitWindow {
            used_percent: 42,
            window_duration_mins: Some(7 * 24 * 60),
            resets_at: Some(1_800_000_000),
        }),
        secondary: None,
        ..snapshot(/*percent*/ 0.0)
    }));
    let reset_label = chat
        .rate_limit_snapshots_by_limit_id
        .get("codex")
        .and_then(|snapshot| snapshot.primary.as_ref())
        .and_then(|window| window.resets_at.as_deref())
        .expect("localized weekly reset label");

    let line = status_line_text(&chat).expect("CxLine footer with weekly usage");

    assert!(line.contains(&format!("42% · {reset_label}")));
}
