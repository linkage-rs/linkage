use crate::Theme;
use iced::button::{Style, StyleSheet};
use iced::Color;

pub fn text(theme: &Theme) -> Text {
    Text { theme: *theme }
}

pub struct Text {
    theme: Theme,
}

impl StyleSheet for Text {
    fn active(&self) -> Style {
        Style {
            text_color: Color {
                a: 0.5,
                ..self.theme.text
            },
            ..Style::default()
        }
    }

    fn hovered(&self) -> Style {
        Style {
            text_color: self.theme.text,
            ..Style::default()
        }
    }

    fn pressed(&self) -> Style {
        self.hovered()
    }

    fn disabled(&self) -> Style {
        Style {
            text_color: Color {
                a: 0.25,
                ..self.theme.text
            },
            ..Style::default()
        }
    }
}
