use crate::Theme;
use iced::pick_list::{Menu, Style, StyleSheet};
use iced::Color;

pub fn themed(theme: &Theme) -> Themed {
    Themed {
        theme: theme.clone(),
    }
}

pub struct Themed {
    theme: Theme,
}

impl StyleSheet for Themed {
    fn menu(&self) -> Menu {
        Menu {
            text_color: Color {
                a: 0.5,
                ..self.theme.text
            },
            background: self.theme.bg.into(),
            border_color: Color {
                a: 0.25,
                ..self.theme.text
            },
            selected_text_color: self.theme.text,
            selected_background: Color {
                a: 0.05,
                ..self.theme.text
            }
            .into(),
            ..Menu::default()
        }
    }

    fn active(&self) -> Style {
        Style {
            text_color: self.theme.text,
            background: self.theme.bg.into(),
            border_color: Color {
                a: 0.1,
                ..self.theme.hit
            },
            ..Style::default()
        }
    }

    fn hovered(&self) -> Style {
        Style {
            border_color: Color {
                a: 0.25,
                ..self.theme.hit
            },
            ..self.active()
        }
    }
}
