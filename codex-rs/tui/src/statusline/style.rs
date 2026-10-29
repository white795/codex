// Derived from Cometix; see this module's mod.rs for provenance.
// 状态栏样式定义
// 参考 CCometixLine 的颜色和样式系统

use ratatui::style::Color;
use serde::Deserialize;
use serde::Serialize;

/// 样式模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StyleMode {
    /// 普通文本模式（使用 emoji）
    Plain,
    /// Nerd Font 模式（使用 Nerd Font 图标）
    #[default]
    NerdFont,
    /// Powerline 模式（带背景色和箭头分隔符）
    Powerline,
}

/// ANSI 颜色（支持 16 色、256 色、RGB）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnsiColor {
    /// 标准 16 色 (0-15)
    #[serde(rename = "c16")]
    Color16 { c16: u8 },
    /// 256 色调色板
    #[serde(rename = "c256")]
    Color256 { c256: u8 },
    /// 24 位真彩色 RGB
    Rgb { r: u8, g: u8, b: u8 },
}

impl AnsiColor {
    /// 创建 16 色
    pub fn c16(code: u8) -> Self {
        Self::Color16 { c16: code }
    }

    /// 创建 256 色
    pub fn c256(code: u8) -> Self {
        Self::Color256 { c256: code }
    }

    /// 创建 RGB 颜色
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    /// 转换为 ratatui Color
    #[allow(clippy::disallowed_methods)] // 颜色系统需要支持 256 色和 RGB
    pub fn to_ratatui_color(self) -> Color {
        match self {
            Self::Color16 { c16 } => match c16 {
                0 => Color::Black,
                1 => Color::Red,
                2 => Color::Green,
                3 => Color::Yellow,
                4 => Color::Blue,
                5 => Color::Magenta,
                6 => Color::Cyan,
                7 => Color::White,
                8 => Color::DarkGray,
                9 => Color::LightRed,
                10 => Color::LightGreen,
                11 => Color::LightYellow,
                12 => Color::LightBlue,
                13 => Color::LightMagenta,
                14 => Color::LightCyan,
                15 => Color::Gray,
                _ => Color::Indexed(c16),
            },
            Self::Color256 { c256 } => Color::Indexed(c256),
            Self::Rgb { r, g, b } => Color::Rgb(r, g, b),
        }
    }
}

/// 预定义 16 色常量
pub mod ansi16 {
    use super::AnsiColor;

    pub const MAGENTA: AnsiColor = AnsiColor::Color16 { c16: 5 };
    pub const BRIGHT_GREEN: AnsiColor = AnsiColor::Color16 { c16: 10 };
    pub const BRIGHT_YELLOW: AnsiColor = AnsiColor::Color16 { c16: 11 };
    pub const BRIGHT_BLUE: AnsiColor = AnsiColor::Color16 { c16: 12 };
    pub const BRIGHT_MAGENTA: AnsiColor = AnsiColor::Color16 { c16: 13 };
    pub const BRIGHT_CYAN: AnsiColor = AnsiColor::Color16 { c16: 14 };
}

/// 图标配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IconConfig {
    /// 普通模式图标（emoji）
    pub plain: String,
    /// Nerd Font 图标
    pub nerd_font: String,
}

impl IconConfig {
    pub fn new(plain: impl Into<String>, nerd_font: impl Into<String>) -> Self {
        Self {
            plain: plain.into(),
            nerd_font: nerd_font.into(),
        }
    }

    /// 根据样式模式获取图标
    pub fn get(&self, mode: StyleMode) -> &str {
        match mode {
            StyleMode::Plain => &self.plain,
            StyleMode::NerdFont | StyleMode::Powerline => &self.nerd_font,
        }
    }
}

/// 颜色配置（支持图标、文本、背景独立配色）
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ColorConfig {
    /// 图标颜色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<AnsiColor>,
    /// 文本颜色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<AnsiColor>,
    /// 背景颜色（主要用于 Powerline 模式）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<AnsiColor>,
}

impl ColorConfig {
    pub fn new(icon: AnsiColor, text: AnsiColor) -> Self {
        Self {
            icon: Some(icon),
            text: Some(text),
            background: None,
        }
    }

    pub fn with_background(mut self, bg: AnsiColor) -> Self {
        self.background = Some(bg);
        self
    }

    /// 获取图标的 ratatui Color
    pub fn icon_color(&self) -> Option<Color> {
        self.icon.map(AnsiColor::to_ratatui_color)
    }

    /// 获取文本的 ratatui Color
    pub fn text_color(&self) -> Option<Color> {
        self.text.map(AnsiColor::to_ratatui_color)
    }

    /// 获取背景的 ratatui Color
    pub fn background_color(&self) -> Option<Color> {
        self.background.map(AnsiColor::to_ratatui_color)
    }
}

/// 文本样式配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TextStyleConfig {
    /// 是否加粗
    #[serde(default)]
    pub text_bold: bool,
}

/// Legacy separators. Custom separator rendering is reviewed separately from migration.
pub mod separators {
    pub const SIMPLE: &str = " │ ";
    pub const POWERLINE_THIN: &str = "\u{e0b1}";
}
