use iced::theme::palette::{Extended, mix};
use iced::theme::{Base, Mode, Palette, Style};
use iced::{Color, color};

/// List all the themes available in the app
pub fn all() -> Vec<Theme> {
    let built_ins = iced::Theme::ALL.iter().cloned().map(Theme::from);
    let linkage = Theme::ALL.iter().cloned();
    let mut all: Vec<Theme> = built_ins.chain(linkage).collect();

    all.sort_by(|a, b| {
        a.palette()
            .background
            .relative_luminance()
            .partial_cmp(&b.palette().background.relative_luminance())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    all
}

#[derive(Debug, Clone)]
pub enum Theme {
    Ayu,
    Monokai,
    NordLight,
    OneDark,
    /// An Iced built-in theme
    BuiltIn(iced::Theme),
}

impl Theme {
    const ALL: &'static [Self] = &[Self::Ayu, Self::Monokai, Self::NordLight, Self::OneDark];

    pub fn name(&self) -> &str {
        match self {
            Theme::Ayu => "Ayu",
            Theme::Monokai => "Monokai",
            Theme::NordLight => "Nord Light",
            Theme::OneDark => "One Dark",
            Theme::BuiltIn(theme) => theme.name(),
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        all().iter().find(|&item| item.name() == name).cloned()
    }

    /// Color of the background
    pub fn background(&self) -> Color {
        self.palette().background
    }

    /// Color of the foreground text
    pub fn text(&self) -> Color {
        self.palette().text
    }

    /// Color of the target proficiency rate
    pub fn target(&self) -> Color {
        self.palette().success
    }

    /// Color of a typed character
    pub fn hit(&self) -> Color {
        self.extended_palette().background.strongest.text
    }

    /// Color of a missed a character
    pub fn miss(&self) -> Color {
        self.palette().warning
    }

    /// Color of an error
    pub fn error(&self) -> Color {
        self.palette().danger
    }

    pub fn palette(&self) -> Palette {
        match self {
            Theme::Ayu => Palette {
                background: color!(0x0a0e14),
                text: color!(0xb3b1ad),
                primary: color!(0x4d5566),
                success: color!(0xc2d94c),
                warning: color!(0xffb454),
                danger: color!(0xf07178),
            },
            Theme::Monokai => Palette {
                background: color!(0x272822),
                text: color!(0xf8f8f2),
                primary: color!(0xcfcfc2),
                success: color!(0xa6e22e),
                warning: color!(0xfd971f),
                danger: color!(0xf92672),
            },
            Theme::NordLight => Palette {
                background: color!(0xeceff4),
                text: color!(0x2e3440),
                primary: color!(0x3b4252),
                success: color!(0x88c0d0),
                warning: color!(0xebcb8b),
                danger: color!(0xbf616a),
            },
            Theme::OneDark => Palette {
                background: color!(0x282c34),
                text: color!(0xabb2bf),
                primary: color!(0x5c6370),
                success: color!(0x98c379),
                warning: color!(0xd19a66),
                danger: color!(0xbe5046),
            },
            Theme::BuiltIn(theme) => theme.palette(),
        }
    }

    pub fn extended_palette(&self) -> Extended {
        Extended::generate(self.palette())
    }

    /// Between 0.0 and 0.5, return a blend from error -> text
    /// Between 0.5 and 1.0, return a blend from text -> target
    pub fn metric(&self, value: f32) -> Color {
        let value = value.min(1.0).max(0.0);
        if value < 0.5 {
            let pct = value / 0.5;
            mix(self.palette().danger, self.palette().text, pct)
        } else {
            let pct = (value - 0.5) / 0.5;
            mix(self.palette().text, self.palette().success, pct)
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Monokai
    }
}

impl Base for Theme {
    fn default(preference: Mode) -> Self {
        match preference {
            Mode::None | Mode::Dark => Theme::Monokai,
            Mode::Light => Theme::NordLight,
        }
    }

    fn mode(&self) -> Mode {
        let palette = self.extended_palette();
        if palette.is_dark {
            Mode::Dark
        } else {
            Mode::Light
        }
    }

    fn base(&self) -> Style {
        let palette = self.palette();
        Style {
            background_color: palette.background,
            text_color: palette.text,
        }
    }

    fn palette(&self) -> Option<Palette> {
        Some(Theme::palette(self))
    }

    fn name(&self) -> &str {
        Theme::name(self)
    }
}

impl From<Theme> for iced::Theme {
    fn from(theme: Theme) -> Self {
        if let Theme::BuiltIn(theme) = theme {
            theme
        } else {
            iced::Theme::custom(theme.name().to_string(), theme.palette())
        }
    }
}

impl From<iced::Theme> for Theme {
    fn from(theme: iced::Theme) -> Self {
        Self::BuiltIn(theme)
    }
}

// impl From<Option<Theme>> for Option<iced::Theme> {
//     fn from(maybe_theme: Option<Theme>) -> Self {
//         maybe_theme.map(iced::Theme::from)
//     }
// }
