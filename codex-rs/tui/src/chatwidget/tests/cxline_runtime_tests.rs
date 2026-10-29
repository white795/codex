//! CxLine live base data, effective Fast state, and official usage windows.

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
    assert!(line.contains("GPT 5.2 Codex · high"));
    assert!(line.contains("cxline-demo"));
    assert!(line.contains("50% · 64.0k tokens"));
    insta::assert_snapshot!("cxline_live_base_data", line);
}

#[tokio::test]
async fn cxline_model_segment_tracks_effective_fast_mode() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.4")).await;
    set_chatgpt_auth(&mut chat);
    set_fast_mode_test_catalog(&mut chat);
    let mut config = CxLineConfig::default();
    config.segments.directory.enabled = false;
    config.segments.git.enabled = false;
    config.segments.context.enabled = false;
    config.segments.usage.enabled = false;
    save_cxline_config(&chat, &config);
    chat.set_reasoning_effort(Some(ReasoningEffort::XHigh));
    chat.apply_cxline_editor_config(config);

    chat.set_service_tier(Some(ServiceTier::Fast.request_value().to_string()));
    assert_eq!(
        status_line_text(&chat),
        Some("\u{e26d} GPT 5.4 · xhigh fast".to_string())
    );
    insta::assert_snapshot!(
        "cxline_live_fast_mode",
        status_line_text(&chat).expect("CxLine footer with Fast mode")
    );

    chat.set_service_tier(Some(SERVICE_TIER_DEFAULT_REQUEST_VALUE.to_string()));
    assert_eq!(
        status_line_text(&chat),
        Some("\u{e26d} GPT 5.4 · xhigh".to_string())
    );
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
async fn cxline_catalog_default_fast_tier_uses_the_effective_tier_and_accepted_spacing() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-6.1-sol")).await;
    set_chatgpt_auth(&mut chat);
    set_fast_mode_test_catalog_for_models(&mut chat, "gpt-6.1-sol", "standard-model");
    let model = Arc::make_mut(&mut chat.model_catalog)
        .models
        .iter_mut()
        .find(|preset| preset.model == "gpt-6.1-sol")
        .expect("fast-capable catalog model");
    model.service_tiers[0].id = "catalog-fast-tier".to_string();
    model.service_tiers[0].name = "FAST".to_string();
    model.default_service_tier = Some("catalog-fast-tier".to_string());
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    chat.set_reasoning_effort(Some(ReasoningEffort::XHigh));

    chat.set_service_tier(/*service_tier*/ None);

    assert_eq!(chat.configured_service_tier(), None);
    assert_eq!(chat.current_service_tier(), Some("catalog-fast-tier"));
    assert!(
        chat.status_line_value_for_item(StatusLineItem::ModelWithReasoning)
            .expect("official model label")
            .ends_with("xhigh fast")
    );
    assert!(
        status_line_text(&chat)
            .expect("CxLine footer")
            .starts_with("\u{e26d} gpt-6.1-sol · xhigh fast")
    );
}

#[tokio::test]
async fn cxline_fast_label_obeys_account_model_and_feature_visibility() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.4")).await;
    set_chatgpt_auth(&mut chat);
    set_fast_mode_test_catalog(&mut chat);
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    chat.set_service_tier(Some(ServiceTier::Fast.request_value().to_string()));
    assert!(status_line_text(&chat).unwrap().contains(" fast"));

    chat.has_chatgpt_account = false;
    chat.refresh_status_line();
    assert!(!status_line_text(&chat).unwrap().contains(" fast"));

    chat.has_chatgpt_account = true;
    chat.set_model("gpt-5.2");
    assert!(!status_line_text(&chat).unwrap().contains(" fast"));
    assert_eq!(chat.current_service_tier(), None);

    chat.set_model("gpt-5.4");
    assert!(status_line_text(&chat).unwrap().contains(" fast"));
    chat.set_feature_enabled(Feature::FastMode, /*enabled*/ false);
    chat.set_service_tier(Some(ServiceTier::Fast.request_value().to_string()));
    assert!(!status_line_text(&chat).unwrap().contains(" fast"));
}

#[tokio::test]
async fn cxline_unknown_model_or_non_fast_catalog_tier_omits_the_fast_label() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.4")).await;
    set_chatgpt_auth(&mut chat);
    set_fast_mode_test_catalog(&mut chat);
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    let model = Arc::make_mut(&mut chat.model_catalog)
        .models
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.4")
        .expect("catalog model");
    model.service_tiers[0].name = "flex".to_string();

    chat.set_service_tier(Some(ServiceTier::Fast.request_value().to_string()));
    assert!(!status_line_text(&chat).unwrap().contains(" fast"));

    chat.set_model("uncatalogued-model");
    assert_eq!(
        chat.current_service_tier(),
        Some(ServiceTier::Fast.request_value())
    );
    assert!(!status_line_text(&chat).unwrap().contains(" fast"));
}

#[tokio::test]
async fn disabling_cxline_restores_official_items_without_changing_the_terminal_title() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-6.1-sol")).await;
    chat.local_settings.tui.status_line = Some(vec!["hostname".to_string()]);
    chat.local_settings.tui.terminal_title = Some(vec!["model".to_string()]);
    chat.refresh_status_line();
    let official_title = chat.last_terminal_title.clone();
    let mut config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config.clone());
    assert!(chat.cxline_enabled());
    assert!(status_line_text(&chat).unwrap().contains("gpt-6.1-sol"));
    assert_eq!(chat.last_terminal_title, official_title);

    config.enabled = false;
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);

    assert!(!chat.cxline_enabled());
    assert_eq!(status_line_text(&chat), codex_config::os_host_name());
    assert_eq!(chat.last_terminal_title, official_title);
    assert_eq!(
        chat.local_settings.tui.status_line,
        Some(vec!["hostname".to_string()])
    );
}

#[tokio::test]
async fn malformed_cxline_config_keeps_the_official_footer_without_repairing_storage() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    chat.local_settings.tui.status_line = Some(vec!["hostname".to_string()]);
    let root = chat.local_settings.codex_home.join("cxline");
    std::fs::create_dir(&root).expect("CxLine directory");
    let content = "enabled = [not toml";
    std::fs::write(root.join("config.toml"), content).expect("malformed configuration");
    chat.cxline_runtime =
        crate::chatwidget::cxline::CxLineRuntime::load(&chat.local_settings.codex_home);

    chat.refresh_status_line();

    assert!(!chat.cxline_enabled());
    assert_eq!(status_line_text(&chat), codex_config::os_host_name());
    assert_eq!(
        std::fs::read_to_string(root.join("config.toml")).unwrap(),
        content
    );
}

#[tokio::test]
async fn cxline_tracks_current_cwd_and_token_updates_without_stale_context_values() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-6.1-sol")).await;
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    chat.current_cwd = Some(std::path::PathBuf::from("/workspace/cxline-new-directory"));

    handle_token_count(
        &mut chat,
        Some(make_token_info(
            /*total_tokens*/ 64_000, /*context_window*/ 128_000,
        )),
    );
    // App::handle_thread_event_now refreshes the status line after the token notification.
    chat.refresh_status_line();
    let known = status_line_text(&chat).expect("CxLine with current context");
    assert!(known.contains("cxline-new-directory"));
    assert!(known.contains("50% · 64.0k tokens"));

    chat.set_token_info(/*info*/ None);
    chat.refresh_status_line();

    let pending = status_line_text(&chat).expect("CxLine with pending context");
    assert!(pending.contains("- · - tokens"));
    assert!(!pending.contains("64.0k"));
}

#[tokio::test]
async fn cxline_uses_matching_usage_windows_when_their_primary_secondary_order_is_reversed() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let config = CxLineConfig::default();
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    let mut usage = paired_usage_snapshot("codex", /*primary*/ 17, /*secondary*/ 42);
    std::mem::swap(&mut usage.primary, &mut usage.secondary);
    chat.on_rate_limit_snapshot(Some(usage));

    let line = status_line_text(&chat).expect("CxLine with reversed usage windows");
    assert!(line.contains("\u{f0aa1} 17%"));
    assert!(!line.contains("42%"));
}

#[tokio::test]
async fn cxline_does_not_start_fetches_for_hidden_official_workspace_headline_items() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    chat.has_codex_backend_auth = true;
    chat.local_settings.tui.status_line = Some(vec!["workspace-headline".to_string()]);
    let mut config = CxLineConfig::default();
    save_cxline_config(&chat, &config);

    chat.apply_cxline_editor_config(config.clone());
    chat.refresh_status_line_if_workspace_headline_due();
    assert!(rx.try_recv().is_err());
    assert!(
        chat.status_line_workspace_headline_pending_request_id
            .is_none()
    );

    config.enabled = false;
    save_cxline_config(&chat, &config);
    chat.apply_cxline_editor_config(config);
    assert_matches!(
        rx.try_recv(),
        Ok(AppEvent::RefreshStatusLineWorkspaceHeadline { .. })
    );
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
