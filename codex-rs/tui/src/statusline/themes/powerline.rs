// Derived from Cometix; see this module's mod.rs for provenance.
use super::*;
use crate::statusline::segment::SegmentId;

impl ThemePresets {
    /// Nord 主题 (Powerline)
    pub fn get_nord() -> CxLineConfig {
        let nord_polar = AnsiColor::rgb(46, 52, 64);
        let bg_model = AnsiColor::rgb(136, 192, 208);
        let bg_dir = AnsiColor::rgb(163, 190, 140);
        let bg_git = AnsiColor::rgb(129, 161, 193);
        let bg_context = AnsiColor::rgb(180, 142, 173);
        let bg_usage = AnsiColor::rgb(235, 203, 139);

        CxLineConfig {
            enabled: true,
            theme: "nord".to_string(),
            style: StyleMode::Powerline,
            separator: "\u{e0b0}".to_string(),
            segments: SegmentsConfig {
                model: SegmentItemConfig {
                    id: SegmentId::Model,
                    enabled: true,
                    icon: IconConfig::new("🤖", "\u{e26d}"),
                    colors: ColorConfig::new(nord_polar, nord_polar).with_background(bg_model),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                directory: SegmentItemConfig {
                    id: SegmentId::Directory,
                    enabled: true,
                    icon: IconConfig::new("📁", "\u{f024b}"),
                    colors: ColorConfig::new(nord_polar, nord_polar).with_background(bg_dir),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                git: SegmentItemConfig {
                    id: SegmentId::Git,
                    enabled: true,
                    icon: IconConfig::new("🌿", "\u{f02a2}"),
                    colors: ColorConfig::new(nord_polar, nord_polar).with_background(bg_git),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                context: SegmentItemConfig {
                    id: SegmentId::Context,
                    enabled: true,
                    icon: IconConfig::new("⚡️", "\u{f49b}"),
                    colors: ColorConfig::new(nord_polar, nord_polar).with_background(bg_context),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                usage: SegmentItemConfig {
                    id: SegmentId::Usage,
                    enabled: true,
                    icon: IconConfig::new("📊", "\u{f0a9e}"),
                    colors: ColorConfig::new(nord_polar, nord_polar).with_background(bg_usage),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
            },
        }
    }

    /// Powerline Dark 主题
    pub fn get_powerline_dark() -> CxLineConfig {
        let white = AnsiColor::rgb(255, 255, 255);
        let light_gray = AnsiColor::rgb(209, 213, 219);

        let bg_model = AnsiColor::rgb(45, 45, 45);
        let bg_dir = AnsiColor::rgb(139, 69, 19);
        let bg_git = AnsiColor::rgb(64, 64, 64);
        let bg_context = AnsiColor::rgb(55, 65, 81);
        let bg_usage = AnsiColor::rgb(45, 50, 59);

        CxLineConfig {
            enabled: true,
            theme: "powerline-dark".to_string(),
            style: StyleMode::Powerline,
            separator: "\u{e0b0}".to_string(),
            segments: SegmentsConfig {
                model: SegmentItemConfig {
                    id: SegmentId::Model,
                    enabled: true,
                    icon: IconConfig::new("🤖", "\u{e26d}"),
                    colors: ColorConfig::new(white, white).with_background(bg_model),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                directory: SegmentItemConfig {
                    id: SegmentId::Directory,
                    enabled: true,
                    icon: IconConfig::new("📁", "\u{f024b}"),
                    colors: ColorConfig::new(white, white).with_background(bg_dir),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                git: SegmentItemConfig {
                    id: SegmentId::Git,
                    enabled: true,
                    icon: IconConfig::new("🌿", "\u{f02a2}"),
                    colors: ColorConfig::new(white, white).with_background(bg_git),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                context: SegmentItemConfig {
                    id: SegmentId::Context,
                    enabled: true,
                    icon: IconConfig::new("⚡️", "\u{f49b}"),
                    colors: ColorConfig::new(light_gray, light_gray).with_background(bg_context),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                usage: SegmentItemConfig {
                    id: SegmentId::Usage,
                    enabled: true,
                    icon: IconConfig::new("📊", "\u{f0a9e}"),
                    colors: ColorConfig::new(light_gray, light_gray).with_background(bg_usage),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
            },
        }
    }

    /// Powerline Light 主题
    pub fn get_powerline_light() -> CxLineConfig {
        let black = AnsiColor::rgb(0, 0, 0);
        let white = AnsiColor::rgb(255, 255, 255);

        let bg_model = AnsiColor::rgb(135, 206, 235);
        let bg_dir = AnsiColor::rgb(255, 107, 71);
        let bg_git = AnsiColor::rgb(79, 179, 217);
        let bg_context = AnsiColor::rgb(107, 114, 128);
        let bg_usage = AnsiColor::rgb(40, 167, 69);

        CxLineConfig {
            enabled: true,
            theme: "powerline-light".to_string(),
            style: StyleMode::Powerline,
            separator: "\u{e0b0}".to_string(),
            segments: SegmentsConfig {
                model: SegmentItemConfig {
                    id: SegmentId::Model,
                    enabled: true,
                    icon: IconConfig::new("🤖", "\u{e26d}"),
                    colors: ColorConfig::new(black, black).with_background(bg_model),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                directory: SegmentItemConfig {
                    id: SegmentId::Directory,
                    enabled: true,
                    icon: IconConfig::new("📁", "\u{f024b}"),
                    colors: ColorConfig::new(white, white).with_background(bg_dir),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                git: SegmentItemConfig {
                    id: SegmentId::Git,
                    enabled: true,
                    icon: IconConfig::new("🌿", "\u{f02a2}"),
                    colors: ColorConfig::new(white, white).with_background(bg_git),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                context: SegmentItemConfig {
                    id: SegmentId::Context,
                    enabled: true,
                    icon: IconConfig::new("⚡️", "\u{f49b}"),
                    colors: ColorConfig::new(white, white).with_background(bg_context),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                usage: SegmentItemConfig {
                    id: SegmentId::Usage,
                    enabled: true,
                    icon: IconConfig::new("📊", "\u{f0a9e}"),
                    colors: ColorConfig::new(white, white).with_background(bg_usage),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
            },
        }
    }

    /// Powerline Rose Pine 主题
    pub fn get_powerline_rose_pine() -> CxLineConfig {
        let rose = AnsiColor::rgb(235, 188, 186);
        let iris = AnsiColor::rgb(196, 167, 231);
        let foam = AnsiColor::rgb(156, 207, 216);
        let subtle = AnsiColor::rgb(224, 222, 244);
        let gold = AnsiColor::rgb(246, 193, 119);

        let bg_model = AnsiColor::rgb(25, 23, 36);
        let bg_dir = AnsiColor::rgb(38, 35, 58);
        let bg_git = AnsiColor::rgb(31, 29, 46);
        let bg_context = AnsiColor::rgb(82, 79, 103);
        let bg_usage = AnsiColor::rgb(35, 33, 54);

        CxLineConfig {
            enabled: true,
            theme: "powerline-rose-pine".to_string(),
            style: StyleMode::Powerline,
            separator: "\u{e0b0}".to_string(),
            segments: SegmentsConfig {
                model: SegmentItemConfig {
                    id: SegmentId::Model,
                    enabled: true,
                    icon: IconConfig::new("🤖", "\u{e26d}"),
                    colors: ColorConfig::new(rose, rose).with_background(bg_model),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                directory: SegmentItemConfig {
                    id: SegmentId::Directory,
                    enabled: true,
                    icon: IconConfig::new("📁", "\u{f024b}"),
                    colors: ColorConfig::new(iris, iris).with_background(bg_dir),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                git: SegmentItemConfig {
                    id: SegmentId::Git,
                    enabled: true,
                    icon: IconConfig::new("🌿", "\u{f02a2}"),
                    colors: ColorConfig::new(foam, foam).with_background(bg_git),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                context: SegmentItemConfig {
                    id: SegmentId::Context,
                    enabled: true,
                    icon: IconConfig::new("⚡️", "\u{f49b}"),
                    colors: ColorConfig::new(subtle, subtle).with_background(bg_context),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                usage: SegmentItemConfig {
                    id: SegmentId::Usage,
                    enabled: true,
                    icon: IconConfig::new("📊", "\u{f0a9e}"),
                    colors: ColorConfig::new(gold, gold).with_background(bg_usage),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
            },
        }
    }

    /// Powerline Tokyo Night 主题
    pub fn get_powerline_tokyo_night() -> CxLineConfig {
        let magenta = AnsiColor::rgb(252, 167, 234);
        let blue = AnsiColor::rgb(130, 170, 255);
        let green = AnsiColor::rgb(195, 232, 141);
        let lavender = AnsiColor::rgb(192, 202, 245);
        let orange = AnsiColor::rgb(224, 175, 104);

        let bg_model = AnsiColor::rgb(25, 27, 41);
        let bg_dir = AnsiColor::rgb(47, 51, 77);
        let bg_git = AnsiColor::rgb(30, 32, 48);
        let bg_context = AnsiColor::rgb(61, 89, 161);
        let bg_usage = AnsiColor::rgb(36, 40, 59);

        CxLineConfig {
            enabled: true,
            theme: "powerline-tokyo-night".to_string(),
            style: StyleMode::Powerline,
            separator: "\u{e0b0}".to_string(),
            segments: SegmentsConfig {
                model: SegmentItemConfig {
                    id: SegmentId::Model,
                    enabled: true,
                    icon: IconConfig::new("🤖", "\u{e26d}"),
                    colors: ColorConfig::new(magenta, magenta).with_background(bg_model),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                directory: SegmentItemConfig {
                    id: SegmentId::Directory,
                    enabled: true,
                    icon: IconConfig::new("📁", "\u{f024b}"),
                    colors: ColorConfig::new(blue, blue).with_background(bg_dir),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                git: SegmentItemConfig {
                    id: SegmentId::Git,
                    enabled: true,
                    icon: IconConfig::new("🌿", "\u{f02a2}"),
                    colors: ColorConfig::new(green, green).with_background(bg_git),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                context: SegmentItemConfig {
                    id: SegmentId::Context,
                    enabled: true,
                    icon: IconConfig::new("⚡️", "\u{f49b}"),
                    colors: ColorConfig::new(lavender, lavender).with_background(bg_context),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
                usage: SegmentItemConfig {
                    id: SegmentId::Usage,
                    enabled: true,
                    icon: IconConfig::new("📊", "\u{f0a9e}"),
                    colors: ColorConfig::new(orange, orange).with_background(bg_usage),
                    styles: TextStyleConfig::default(),
                    options: HashMap::new(),
                },
            },
        }
    }
}
