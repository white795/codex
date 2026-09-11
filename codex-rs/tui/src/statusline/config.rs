// Derived from Cometix; see this module's mod.rs for provenance.
// 状态栏配置
// Configuration is read beneath the caller's resolved CODEX_HOME.

use super::segment::SegmentId;
use super::style::ColorConfig;
use super::style::IconConfig;
use super::style::StyleMode;
use super::style::TextStyleConfig;
use super::themes::ThemePresets;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

/// 状态栏配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CxLineConfig {
    /// 是否启用状态栏
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// 当前使用的主题名称
    #[serde(default = "default_theme")]
    pub theme: String,

    /// 样式模式
    #[serde(default)]
    pub style: StyleMode,

    /// 分隔符（仅 Plain/NerdFont 模式使用）
    #[serde(default = "default_separator")]
    pub separator: String,

    /// 各 segment 配置
    #[serde(default)]
    pub segments: SegmentsConfig,
}

fn default_true() -> bool {
    true
}

fn default_theme() -> String {
    "cometix".to_string()
}

fn default_separator() -> String {
    " │ ".to_string()
}

/// 各 segment 的配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentsConfig {
    #[serde(default = "SegmentItemConfig::default_model")]
    pub model: SegmentItemConfig,

    #[serde(default = "SegmentItemConfig::default_directory")]
    pub directory: SegmentItemConfig,

    #[serde(default = "SegmentItemConfig::default_git")]
    pub git: SegmentItemConfig,

    #[serde(default = "SegmentItemConfig::default_context")]
    pub context: SegmentItemConfig,

    #[serde(default = "SegmentItemConfig::default_usage")]
    pub usage: SegmentItemConfig,
}

impl Default for SegmentsConfig {
    fn default() -> Self {
        let theme = ThemePresets::get_default();
        theme.segments
    }
}

/// 单个 segment 的配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentItemConfig {
    /// Segment ID
    #[serde(default)]
    pub id: SegmentId,

    /// 是否启用
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// 图标配置
    #[serde(default)]
    pub icon: IconConfig,

    /// 颜色配置
    #[serde(default)]
    pub colors: ColorConfig,

    /// 文本样式配置
    #[serde(default)]
    pub styles: TextStyleConfig,

    /// 自定义选项
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub options: HashMap<String, serde_json::Value>,
}

impl SegmentItemConfig {
    pub fn default_model() -> Self {
        ThemePresets::get_default().segments.model
    }

    pub fn default_directory() -> Self {
        ThemePresets::get_default().segments.directory
    }

    pub fn default_git() -> Self {
        ThemePresets::get_default().segments.git
    }

    pub fn default_context() -> Self {
        ThemePresets::get_default().segments.context
    }

    pub fn default_usage() -> Self {
        ThemePresets::get_default().segments.usage
    }
}

impl Default for CxLineConfig {
    fn default() -> Self {
        ThemePresets::get_cometix()
    }
}

impl CxLineConfig {
    /// Read config from the resolved client home without initializing or changing files.
    ///
    /// Missing files use built-in defaults. Other failures are returned to the UI so it
    /// can report them and choose a fallback without overwriting the user's configuration.
    pub fn load(codex_home: &Path) -> io::Result<Self> {
        let path = codex_home.join("cxline").join("config.toml");
        match fs::read_to_string(path) {
            Ok(content) => toml::from_str(&content)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(error),
        }
    }

    /// Return configuration for a rendered segment.
    pub fn get_segment_config(&self, id: SegmentId) -> &SegmentItemConfig {
        match id {
            SegmentId::Model => &self.segments.model,
            SegmentId::Directory => &self.segments.directory,
            SegmentId::Git => &self.segments.git,
            SegmentId::Context => &self.segments.context,
            SegmentId::Usage => &self.segments.usage,
        }
    }
}
