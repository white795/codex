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
