// Derived from Cometix; see this module's mod.rs for provenance.
// Directory Segment - 显示当前工作目录名称

use crate::statusline::StatusLineContext;
use crate::statusline::segment::Segment;
use crate::statusline::segment::SegmentData;

pub struct DirectorySegment;

impl Segment for DirectorySegment {
    fn collect(&self, ctx: &StatusLineContext) -> Option<SegmentData> {
        let cwd = ctx.cwd;
        let dir_name = extract_directory_name(cwd);

        if dir_name.is_empty() {
            return None;
        }

        Some(SegmentData::new(&dir_name).with_metadata("full_path", cwd.to_string_lossy()))
    }
}

/// 提取目录名称
/// 支持 Unix 和 Windows 路径
fn extract_directory_name(path: &std::path::Path) -> String {
    // 获取最后一个组件（目录名）
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            // 如果是根目录，返回 "/"
            if path.as_os_str().is_empty() {
                String::new()
            } else {
                "/".to_string()
            }
        })
}

#[cfg(test)]
#[path = "directory_tests.rs"]
mod tests;
