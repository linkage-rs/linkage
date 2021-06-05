use crate::Theme;
use iced::container::{Style, StyleSheet};

pub fn primary(theme: &Theme) -> Primary {
    Primary { theme: *theme }
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
