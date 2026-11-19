// Derived from Cometix; see this module's mod.rs for provenance.
// 状态栏 Segment 定义
// 参考 CCometixLine 的设计模式

use std::collections::HashMap;

/// Segment 数据，由各 Segment 实现收集后返回
#[derive(Debug, Clone, Default)]
pub struct SegmentData {
    /// 主要内容
    pub primary: String,
    /// 次要内容（可选，通常在主内容后显示）
    pub secondary: String,
    /// 元数据（用于动态图标等）
    pub metadata: HashMap<String, String>,
}

impl SegmentData {
    pub fn new(primary: impl Into<String>) -> Self {
        Self {
            primary: primary.into(),
            secondary: String::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_secondary(mut self, secondary: impl Into<String>) -> Self {
        self.secondary = secondary.into();
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Segment ID 枚举
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SegmentId {
    #[default]
    Model,
    Directory,
    Git,
    Context,
    Usage,
}

/// Segment trait，所有 segment 实现此 trait
pub trait Segment {
    /// 收集 segment 数据
    fn collect(&self, ctx: &super::StatusLineContext) -> Option<SegmentData>;
}
