//! CxLine configuration, editing, and footer rendering.
//!
//! Derived from Haleclipse/codex, rust-v0.144.3-cometix,
//! commit c5dce3cbd3914c6a3402fb00b2ee9d4df988b0a6 (Apache-2.0).
//! The original implementation credits CCometixLine's design.
//! Migration changes separate filesystem access from defaults and rendering. Git here only renders
//! supplied data; background command execution is a later stage.

mod color_picker;
mod config;
mod editor;
mod icon_selector;
mod name_input;
mod overlay;
mod renderer;
mod segment;
mod segments;
mod separator_editor;
mod storage;
mod style;
mod themes;

use codex_protocol::openai_models::ReasoningEffort;
pub(crate) use config::CxLineConfig;
pub(crate) use overlay::CxLineOverlay;
use renderer::StatusLineRenderer;
use segment::Segment;
use segment::SegmentId;
use std::path::Path;

#[cfg(test)]
#[path = "foundation_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "storage_tests.rs"]
mod storage_tests;

#[cfg(test)]
#[path = "text_editor_tests.rs"]
mod text_editor_tests;

#[cfg(test)]
#[path = "overlay_tests.rs"]
mod cxline_overlay_tests;

/// Git 预览数据（用于配置页预览）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitPreviewData {
    pub branch: String,
    pub status: String,
    pub ahead: u32,
    pub behind: u32,
}

/// 状态栏数据上下文
/// 包含渲染状态栏所需的所有数据
pub struct StatusLineContext<'a> {
    /// 当前模型名称
    pub model_name: &'a str,

    /// Reasoning effort level
    pub reasoning_effort: Option<ReasoningEffort>,

    /// 当前工作目录
    pub cwd: &'a Path,

    /// 已使用的 token 数
    pub context_used_tokens: Option<i64>,

    /// 上下文窗口大小（用于计算使用占比）
    pub context_window_size: Option<i64>,

    /// 5h Rate limit 使用百分比 (用于百分比数字显示)
    pub hourly_rate_limit_percent: Option<f64>,

    /// Weekly Rate limit 使用百分比 (用于圆圈进度条)
    pub weekly_rate_limit_percent: Option<f64>,

    /// Weekly Rate limit 重置时间
    pub weekly_rate_limit_resets_at: Option<String>,

    /// Git 预览数据（用于配置页预览，覆盖实际 git 检测）
    pub git_preview: Option<GitPreviewData>,
}

impl<'a> StatusLineContext<'a> {
    pub fn new(model_name: &'a str, cwd: &'a Path) -> Self {
        Self {
            model_name,
            reasoning_effort: None,
            cwd,
            context_used_tokens: None,
            context_window_size: None,
            hourly_rate_limit_percent: None,
            weekly_rate_limit_percent: None,
            weekly_rate_limit_resets_at: None,
            git_preview: None,
        }
    }

    pub fn with_reasoning_effort(mut self, effort: Option<ReasoningEffort>) -> Self {
        self.reasoning_effort = effort;
        self
    }

    pub fn with_context(mut self, used_tokens: Option<i64>, window_size: Option<i64>) -> Self {
        self.context_used_tokens = used_tokens;
        self.context_window_size = window_size;
        self
    }

    pub fn with_rate_limit(
        mut self,
        hourly_percent: Option<f64>,
        weekly_percent: Option<f64>,
        weekly_resets_at: Option<String>,
    ) -> Self {
        self.hourly_rate_limit_percent = hourly_percent;
        self.weekly_rate_limit_percent = weekly_percent;
        self.weekly_rate_limit_resets_at = weekly_resets_at;
        self
    }

    /// 设置 Git 预览数据（用于配置页预览）
    pub fn with_git_preview(mut self, branch: &str, status: &str, ahead: u32, behind: u32) -> Self {
        self.git_preview = Some(GitPreviewData {
            branch: branch.to_string(),
            status: status.to_string(),
            ahead,
            behind,
        });
        self
    }
}

/// 构建状态栏
/// 收集所有 segment 数据并返回渲染器
pub(crate) fn build_statusline<'a>(
    config: &'a CxLineConfig,
    ctx: &StatusLineContext<'_>,
) -> StatusLineRenderer<'a> {
    use segments::*;

    let mut renderer = StatusLineRenderer::new(config);

    // Model segment
    if config.segments.model.enabled {
        let segment = ModelSegment;
        if let Some(data) = segment.collect(ctx) {
            renderer.add_segment(SegmentId::Model, data);
        }
    }

    // Directory segment
    if config.segments.directory.enabled {
        let segment = DirectorySegment;
        if let Some(data) = segment.collect(ctx) {
            renderer.add_segment(SegmentId::Directory, data);
        }
    }

    // Git segment
    if config.segments.git.enabled {
        let segment = GitSegment;
        if let Some(data) = segment.collect(ctx) {
            renderer.add_segment(SegmentId::Git, data);
        }
    }

    // Context segment
    if config.segments.context.enabled {
        let segment = ContextSegment;
        if let Some(data) = segment.collect(ctx) {
            renderer.add_segment(SegmentId::Context, data);
        }
    }

    // Usage segment
    if config.segments.usage.enabled {
        let segment = UsageSegment;
        if let Some(data) = segment.collect(ctx) {
            renderer.add_segment(SegmentId::Usage, data);
        }
    }

    renderer
}
