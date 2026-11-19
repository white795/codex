use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_get_circle_icon() {
    // 测试边界值
    assert_eq!(get_circle_icon(0.0), "\u{f0a9e}");
    assert_eq!(get_circle_icon(0.5), "\u{f0aa1}");
    assert_eq!(get_circle_icon(1.0), "\u{f0aa5}");
}
