use crate::Theme;
use iced::button::{Style, StyleSheet};
use iced::Color;

pub fn text(theme: &Theme) -> Text {
    Text { theme: *theme }
}

pub fn menu(theme: &Theme, selected: bool) -> Menu {
    Menu {
        theme: *theme,
        selected,
    }
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

pub struct Menu {
    theme: Theme,
    selected: bool,
}

impl StyleSheet for Menu {
    fn active(&self) -> Style {
        Style {
            text_color: if self.selected {
                self.theme.text
            } else {
                Color {
                    a: 0.5,
                    ..self.theme.text
                }
            },
            background: self.selected.then(|| {
                Color {
                    a: 0.05,
                    ..self.theme.text
                }
                .into()
            }),
            ..Style::default()
        }
    }

    fn hovered(&self) -> Style {
        Style {
            text_color: if self.selected {
                self.theme.text
            } else {
                Color {
                    a: 0.75,
                    ..self.theme.text
                }
            },
            background: self.selected.then(|| {
                Color {
                    a: 0.075,
                    ..self.theme.text
                }
                .into()
            }),
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
