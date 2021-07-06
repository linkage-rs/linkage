use crate::Theme;
use iced::rule::{FillMode, Style, StyleSheet};
use iced::Color;

pub fn divider(theme: &Theme) -> Divider {
    Divider { theme: *theme }
}

pub struct Divider {
    theme: Theme,
}

impl StyleSheet for Divider {
    fn style(&self) -> Style {
        Style {
            color: Color {
                a: 0.25,
                ..self.theme.hit
            },
            fill_mode: FillMode::Full,
            ..Style::default()
        }
    }
}
