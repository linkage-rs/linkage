use iced::Color;
use palette::{Mix, Srgb};

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub bg: Color,
    pub text: Color,
    pub target: Color,
    pub hit: Color,
    pub miss: Color,
    pub error: Color,
}

impl Theme {
    pub fn all() -> Vec<Self> {
        vec![Self::ayu(), Self::monokai()]
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::all().iter().find(|&item| item.name == name).cloned()
    }

    pub fn monokai() -> Self {
        Self {
            name: "Monokai".to_string(),
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
            name: "Ayu".to_string(),
            bg: Color::from_rgba8(0x0A, 0x0E, 0x14, 1.0),
            text: Color::from_rgba8(0xB3, 0xB1, 0xAD, 1.0),
            target: Color::from_rgba8(0xc2, 0xd9, 0x4c, 1.0),
            hit: Color::from_rgba8(0x4D, 0x55, 0x66, 1.0),
            miss: Color::from_rgba8(0xFF, 0xB4, 0x54, 1.0),
            error: Color::from_rgba8(0xF0, 0x71, 0x78, 1.0),
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
            return Srgb::from_linear(error.mix(&text, pct)).into();
        } else {
            let pct = (value - 0.5) / 0.5;
            let target = Srgb::from(self.target).into_linear();
            return Srgb::from_linear(text.mix(&target, pct)).into();
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::monokai()
    }
}
