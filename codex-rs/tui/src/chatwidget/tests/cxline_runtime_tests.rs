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
