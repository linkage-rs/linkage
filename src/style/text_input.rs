use crate::Theme;
use iced::text_input::{Style, StyleSheet};
use iced::Color;

pub fn themed(theme: &Theme) -> Themed {
    Themed {
        theme: theme.clone(),
    }
}

pub struct Themed {
    theme: Theme,
}

// pub struct Style {
//     pub background: Background,
//     pub border_radius: f32,
//     pub border_width: f32,
//     pub border_color: Color,
// }

impl StyleSheet for Themed {
    fn active(&self) -> Style {
        Style {
            background: Color::TRANSPARENT.into(),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.1,
                ..self.theme.hit
            },
        }
    }

    fn focused(&self) -> Style {
        Style {
            background: Color {
                a: 0.025,
                ..self.theme.hit
            }
            .into(),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.25,
                ..self.theme.hit
            },
        }
    }

    fn placeholder_color(&self) -> Color {
        Color {
            a: 0.25,
            ..self.theme.hit
        }
    }

    fn selection_color(&self) -> Color {
        Color {
            a: 0.05,
            ..self.theme.text
        }
    }

    fn value_color(&self) -> Color {
        self.theme.text
    }
}
