use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_simplify_model_name() {
    // 测试日期后缀移除
    assert_eq!(
        simplify_model_name("gpt-5.2-codex-2025-01-15"),
        "GPT 5.2 Codex"
    );
    assert_eq!(
        simplify_model_name("gpt-5.1-codex-max-20250101"),
        "GPT 5.1 Codex Max"
    );
    // 测试模型名称映射
    assert_eq!(simplify_model_name("gpt-5.4"), "GPT 5.4");
    assert_eq!(simplify_model_name("gpt-5.2-codex"), "GPT 5.2 Codex");
    assert_eq!(
        simplify_model_name("gpt-5.1-codex-max"),
        "GPT 5.1 Codex Max"
    );
    assert_eq!(simplify_model_name("gpt-5"), "GPT 5");
    // 测试无映射的模型
    assert_eq!(simplify_model_name("custom-model"), "custom-model");
}

#[test]
fn fast_mode_follows_reasoning_effort_and_preserves_the_model_id() {
    for (effort, expected) in [
        (None, "gpt-6.1-sol"),
        (Some(ReasoningEffort::None), "gpt-6.1-sol"),
        (Some(ReasoningEffort::XHigh), "gpt-6.1-sol · xhigh"),
        (Some(ReasoningEffort::Max), "gpt-6.1-sol · max"),
        (Some(ReasoningEffort::Ultra), "gpt-6.1-sol · ultra"),
    ] {
        let ctx = StatusLineContext::new("gpt-6.1-sol", std::path::Path::new(""))
            .with_reasoning_effort(effort);
        let regular = ModelSegment.collect(&ctx).unwrap();
        assert_eq!(regular.primary, expected);
        assert_eq!(regular.metadata.get("model_id").unwrap(), "gpt-6.1-sol");

        let fast = ModelSegment.collect(&ctx.with_fast_mode_active()).unwrap();
        assert_eq!(fast.primary, format!("{expected} fast"));
        assert_eq!(fast.metadata.get("model_id").unwrap(), "gpt-6.1-sol");
    }
}
