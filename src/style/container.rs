use crate::Theme;
use iced::container::{Style, StyleSheet};
use iced::Color;

pub fn primary(theme: &Theme) -> Primary {
    Primary {
        theme: theme.clone(),
    }
}

pub fn menu_selected(theme: &Theme) -> MenuSelected {
    MenuSelected {
        theme: theme.clone(),
    }
}

pub struct Primary {
    theme: Theme,
}

impl StyleSheet for Primary {
    fn style(&self) -> Style {
        Style {
            text_color: Some(self.theme.text),
            background: Some(self.theme.bg.into()),
            ..Style::default()
        }
    }
}

/// Use for selected menu items that aren't clickable
pub struct MenuSelected {
    theme: Theme,
}

impl StyleSheet for MenuSelected {
    fn style(&self) -> Style {
        Style {
            text_color: Some(self.theme.text),
            background: Some(
                Color {
                    a: 0.05,
                    ..self.theme.text
                }
                .into(),
            ),
            ..Style::default()
        }
    }
}
