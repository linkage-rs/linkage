use iced::Color;
use palette::{Mix, Srgb};

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub text: Color,
    pub target: Color,
    pub hit: Color,
    pub miss: Color,
    pub error: Color,
}

impl Theme {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ayu(),
            Self::monokai(),
            Self::tokyo_night(),
            Self::one_dark(),
        ]
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::all().iter().find(|&item| item.name == name).cloned()
    }

    pub fn monokai() -> Self {
        Self {
            name: "Monokai",
            bg: Color::from_rgba8(0x27, 0x28, 0x22, 1.0),
            text: Color::from_rgba8(0xf8, 0xf8, 0xf2, 1.0),
            target: Color::from_rgba8(0xa6, 0xe2, 0x2e, 1.0),
            hit: Color::from_rgba8(0xcf, 0xcf, 0xc2, 1.0),
            miss: Color::from_rgba8(0xfd, 0x97, 0x1f, 1.0),
            error: Color::from_rgba8(0xf9, 0x26, 0x72, 1.0),
        }
    }

    pub fn ayu() -> Self {
        Self {
            name: "Ayu",
            bg: Color::from_rgba8(0x0A, 0x0E, 0x14, 1.0),
            text: Color::from_rgba8(0xB3, 0xB1, 0xAD, 1.0),
            target: Color::from_rgba8(0xc2, 0xd9, 0x4c, 1.0),
            hit: Color::from_rgba8(0x4D, 0x55, 0x66, 1.0),
            miss: Color::from_rgba8(0xFF, 0xB4, 0x54, 1.0),
            error: Color::from_rgba8(0xF0, 0x71, 0x78, 1.0),
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night",
            bg: Color::from_rgba8(0x1A, 0x1B, 0x26, 1.0),
            text: Color::from_rgba8(0xC0, 0xCA, 0xF5, 1.0),
            target: Color::from_rgba8(0x9E, 0xCE, 0x6A, 1.0),
            hit: Color::from_rgba8(0x56, 0x5F, 0x89, 1.0),
            miss: Color::from_rgba8(0xFF, 0x9E, 0x64, 1.0),
            error: Color::from_rgba8(0xF7, 0x76, 0x8E, 1.0),
        }
    }

    pub fn one_dark() -> Self {
        Self {
            name: "One Dark",
            bg: Color::from_rgba8(0x28, 0x2C, 0x34, 1.0),
            text: Color::from_rgba8(0xAB, 0xB2, 0xBF, 1.0),
            target: Color::from_rgba8(0x98, 0xC3, 0x79, 1.0),
            hit: Color::from_rgba8(0x5C, 0x63, 0x70, 1.0),
            miss: Color::from_rgba8(0xD1, 0x9A, 0x66, 1.0),
            error: Color::from_rgba8(0xBE, 0x50, 0x46, 1.0),
        }
    }

    /// Between 0.0 and 0.5, return a blend from error -> text
    /// Between 0.5 and 1.0, return a blend from text -> target
    pub fn metric(&self, value: f32) -> Color {
        let value = value.min(1.0).max(0.0);
        let text = Srgb::from(self.text).into_linear();
        if value < 0.5 {
            let pct = value / 0.5;
            let error = Srgb::from(self.error).into_linear();
            Srgb::from_linear(error.mix(&text, pct)).into()
        } else {
            let pct = (value - 0.5) / 0.5;
            let target = Srgb::from(self.target).into_linear();
            Srgb::from_linear(text.mix(&target, pct)).into()
        }
    }
}

pub fn alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::monokai()
    }
}
