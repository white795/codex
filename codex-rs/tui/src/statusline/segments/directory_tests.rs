use super::*;
use pretty_assertions::assert_eq;
use std::path::Path;

#[test]
fn test_extract_directory_name() {
    // Unix 路径测试
    assert_eq!(
        extract_directory_name(Path::new("/home/user/projects/codex")),
        "codex"
    );
    assert_eq!(extract_directory_name(Path::new("/home/user")), "user");

    // 根目录
    assert_eq!(extract_directory_name(Path::new("/")), "/");

    // 相对路径
    assert_eq!(extract_directory_name(Path::new("some/path")), "path");
}
