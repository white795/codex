use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_format_tokens() {
    assert_eq!(format_tokens(500), "500");
    assert_eq!(format_tokens(1500), "1.5k");
    assert_eq!(format_tokens(150000), "150.0k");
    assert_eq!(format_tokens(1500000), "1.5M");
}
