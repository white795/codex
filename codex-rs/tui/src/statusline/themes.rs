// Derived from Cometix; see this module's mod.rs for provenance.
// 主题预设系统

use super::config::CxLineConfig;
use super::config::SegmentItemConfig;
use super::config::SegmentsConfig;
use super::style::AnsiColor;
use super::style::ColorConfig;
use super::style::IconConfig;
use super::style::StyleMode;
use super::style::TextStyleConfig;
use super::style::ansi16;
use std::collections::HashMap;

mod powerline;

/// Built-in themes only; construction never consults the filesystem.
pub struct ThemePresets;

impl ThemePresets {
    /// 获取内置预设主题
    pub fn get_builtin(theme_name: &str) -> Option<CxLineConfig> {
        match theme_name {
            "default" => Some(Self::get_default()),
            "cometix" => Some(Self::get_cometix()),
            "minimal" => Some(Self::get_minimal()),
            "gruvbox" => Some(Self::get_gruvbox()),
            "nord" => Some(Self::get_nord()),
            "powerline-dark" => Some(Self::get_powerline_dark()),
            "powerline-light" => Some(Self::get_powerline_light()),
            "powerline-rose-pine" => Some(Self::get_powerline_rose_pine()),
            "powerline-tokyo-night" => Some(Self::get_powerline_tokyo_night()),
            _ => None,
        }
    }

    /// Default 主题
    pub fn get_default() -> CxLineConfig {
        CxLineConfig {
            enabled: true,
            theme: "default".to_string(),
            style: StyleMode::Plain,
            separator: " │ ".to_string(),
            segments: SegmentsConfig {
                model: SegmentItemConfig {
                    id: super::segment::SegmentId::Model,
                    enabled: true,
                    icon: IconConfig::new("🤖", "\u{e26d}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_CYAN, ansi16::BRIGHT_CYAN),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                directory: SegmentItemConfig {
                    id: super::segment::SegmentId::Directory,
                    enabled: true,
                    icon: IconConfig::new("📁", "\u{f024b}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_YELLOW, ansi16::BRIGHT_GREEN),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                git: SegmentItemConfig {
                    id: super::segment::SegmentId::Git,
                    enabled: true,
                    icon: IconConfig::new("🌿", "\u{f02a2}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_BLUE, ansi16::BRIGHT_BLUE),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                context: SegmentItemConfig {
                    id: super::segment::SegmentId::Context,
                    enabled: true,
                    icon: IconConfig::new("⚡️", "\u{f49b}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_MAGENTA, ansi16::BRIGHT_MAGENTA),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                usage: SegmentItemConfig {
                    id: super::segment::SegmentId::Usage,
                    enabled: true,
                    icon: IconConfig::new("📊", "\u{f0a9e}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_CYAN, ansi16::BRIGHT_CYAN),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
            },
        }
    }

    /// Cometix 主题
    pub fn get_cometix() -> CxLineConfig {
        CxLineConfig {
            enabled: true,
            theme: "cometix".to_string(),
            style: StyleMode::NerdFont,
            separator: " │ ".to_string(),
            segments: SegmentsConfig {
                model: SegmentItemConfig {
                    id: super::segment::SegmentId::Model,
                    enabled: true,
                    icon: IconConfig::new("🤖", "\u{e26d}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_CYAN, ansi16::BRIGHT_CYAN),
                    styles: TextStyleConfig { text_bold: true },
                    options: HashMap::new(),
                },
                directory: SegmentItemConfig {
                    id: super::segment::SegmentId::Directory,
                    enabled: true,
                    icon: IconConfig::new("📁", "\u{f024b}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_YELLOW, ansi16::BRIGHT_GREEN),
                    styles: TextStyleConfig { text_bold: true },
                    options: HashMap::new(),
                },
                git: SegmentItemConfig {
                    id: super::segment::SegmentId::Git,
                    enabled: true,
                    icon: IconConfig::new("🌿", "\u{f02a2}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_BLUE, ansi16::BRIGHT_BLUE),
                    styles: TextStyleConfig { text_bold: true },
                    options: HashMap::new(),
                },
                context: SegmentItemConfig {
                    id: super::segment::SegmentId::Context,
                    enabled: true,
                    icon: IconConfig::new("⚡️", "\u{f49b}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_MAGENTA, ansi16::BRIGHT_MAGENTA),
                    styles: TextStyleConfig { text_bold: true },
                    options: HashMap::new(),
                },
                usage: SegmentItemConfig {
                    id: super::segment::SegmentId::Usage,
                    enabled: true,
                    icon: IconConfig::new("📊", "\u{f0a9e}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_CYAN, ansi16::BRIGHT_CYAN),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
            },
        }
    }

    /// Minimal 主题
    pub fn get_minimal() -> CxLineConfig {
        CxLineConfig {
            enabled: true,
            theme: "minimal".to_string(),
            style: StyleMode::Plain,
            separator: " │ ".to_string(),
            segments: SegmentsConfig {
                model: SegmentItemConfig {
                    id: super::segment::SegmentId::Model,
                    enabled: true,
                    icon: IconConfig::new("✽", "\u{f2d0}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_CYAN, ansi16::BRIGHT_CYAN),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                directory: SegmentItemConfig {
                    id: super::segment::SegmentId::Directory,
                    enabled: true,
                    icon: IconConfig::new("◐", "\u{f024b}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_YELLOW, ansi16::BRIGHT_GREEN),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                git: SegmentItemConfig {
                    id: super::segment::SegmentId::Git,
                    enabled: true,
                    icon: IconConfig::new("※", "\u{f02a2}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_BLUE, ansi16::BRIGHT_BLUE),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                context: SegmentItemConfig {
                    id: super::segment::SegmentId::Context,
                    enabled: true,
                    icon: IconConfig::new("◐", "\u{f49b}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_MAGENTA, ansi16::BRIGHT_MAGENTA),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                usage: SegmentItemConfig {
                    id: super::segment::SegmentId::Usage,
                    enabled: true,
                    icon: IconConfig::new("📊", "\u{f0a9e}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_CYAN, ansi16::BRIGHT_CYAN),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
            },
        }
    }

    /// Gruvbox 主题
    pub fn get_gruvbox() -> CxLineConfig {
        let gruvbox_orange = AnsiColor::c256(208);
        let gruvbox_green = AnsiColor::c256(142);
        let gruvbox_cyan = AnsiColor::c256(109);

        CxLineConfig {
            enabled: true,
            theme: "gruvbox".to_string(),
            style: StyleMode::NerdFont,
            separator: " │ ".to_string(),
            segments: SegmentsConfig {
                model: SegmentItemConfig {
                    id: super::segment::SegmentId::Model,
                    enabled: true,
                    icon: IconConfig::new("🤖", "\u{e26d}"),
                    colors: ColorConfig::new(gruvbox_orange, gruvbox_orange),
                    styles: TextStyleConfig { text_bold: true },
                    options: HashMap::new(),
                },
                directory: SegmentItemConfig {
                    id: super::segment::SegmentId::Directory,
                    enabled: true,
                    icon: IconConfig::new("📁", "\u{f024b}"),
                    colors: ColorConfig::new(gruvbox_green, gruvbox_green),
                    styles: TextStyleConfig { text_bold: true },
                    options: HashMap::new(),
                },
                git: SegmentItemConfig {
                    id: super::segment::SegmentId::Git,
                    enabled: true,
                    icon: IconConfig::new("🌿", "\u{f02a2}"),
                    colors: ColorConfig::new(gruvbox_cyan, gruvbox_cyan),
                    styles: TextStyleConfig { text_bold: true },
                    options: HashMap::new(),
                },
                context: SegmentItemConfig {
                    id: super::segment::SegmentId::Context,
                    enabled: true,
                    icon: IconConfig::new("⚡️", "\u{f49b}"),
                    colors: ColorConfig::new(ansi16::MAGENTA, ansi16::MAGENTA),
                    styles: TextStyleConfig { text_bold: true },
                    options: HashMap::new(),
                },
                usage: SegmentItemConfig {
                    id: super::segment::SegmentId::Usage,
                    enabled: true,
                    icon: IconConfig::new("📊", "\u{f0a9e}"),
                    colors: ColorConfig::new(ansi16::BRIGHT_CYAN, ansi16::BRIGHT_CYAN),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
            },
        }
    }
}
