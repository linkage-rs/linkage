// pub mod button;
// pub mod container;
// pub mod pick_list;
// pub mod rule;
// pub mod text_input;

use iced::widget::overlay::menu;
use iced::widget::{button, container, pick_list, rule, scrollable, text, text_input};
use iced::{Background, Color};

use crate::data;
use crate::data::theme::alpha;

#[derive(Debug, Default, Clone)]
pub struct Theme {
    theme: data::Theme,
}

impl Theme {
    pub fn new(theme: data::Theme) -> Self {
        Self { theme }
    }

    pub fn name(&self) -> &'static str {
        self.theme.name
    }
}

impl iced::application::StyleSheet for Theme {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: self.theme.bg,
            text_color: self.theme.text,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
    Error,
    Metric(f32),
    Miss,
    Override(Color),
    Target,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        let color = match style {
            Text::Default => None,
            Text::Error => Some(self.theme.error),
            Text::Metric(metric) => Some(self.theme.metric(metric)),
            Text::Miss => Some(self.theme.miss),
            Text::Override(color) => Some(color),
            Text::Target => Some(self.theme.target),
        };

        text::Appearance { color }
    }
}

#[derive(Default)]
pub enum Button {
    /// Accept changes
    Accept,
    /// Normal button
    #[default]
    Basic,
    /// Menu item
    Menu { selected: bool },
    /// Reject changes
    Reject,
    /// Bare text
    Text,
    /// Override theme
    ThemePreview(data::Theme),
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let appearance = button::Appearance {
            border_radius: 2.0,
            border_width: 1.0,
            border_color: alpha(self.theme.text, 0.15),
            text_color: alpha(self.theme.text, 0.75),
            ..button::Appearance::default()
        };

        match style {
            Button::Accept => button::Appearance {
                background: alpha(self.theme.target, 0.05).into(),
                border_color: alpha(self.theme.target, 0.15),
                text_color: alpha(self.theme.target, 0.75),
                ..appearance
            },
            Button::Basic => appearance,
            Button::Menu { selected } => button::Appearance {
                background: selected.then(|| alpha(self.theme.text, 0.05).into()),
                text_color: if *selected {
                    self.theme.text
                } else {
                    alpha(self.theme.text, 0.5)
                },
                ..button::Appearance::default()
            },
            Button::Reject => button::Appearance {
                background: alpha(self.theme.error, 0.05).into(),
                border_color: alpha(self.theme.error, 0.15),
                text_color: alpha(self.theme.error, 0.75),
                ..appearance
            },
            Button::Text => button::Appearance {
                text_color: alpha(self.theme.text, 0.5),
                border_width: 0.0,
                ..appearance
            },
            Button::ThemePreview(theme) => button::Appearance {
                border_color: alpha(theme.text, 0.15),
                text_color: alpha(theme.text, 0.75),
                ..appearance
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let appearance = button::Appearance {
            background: Some(alpha(self.theme.text, 0.025).into()),
            border_color: alpha(self.theme.text, 0.5),
            text_color: self.theme.text,
            ..self.active(style)
        };

        match style {
            Button::Accept => button::Appearance {
                background: alpha(self.theme.target, 0.1).into(),
                border_color: alpha(self.theme.target, 0.5),
                text_color: self.theme.target,
                ..appearance
            },
            Button::Basic => appearance,
            Button::Menu { selected } => button::Appearance {
                background: selected.then(|| alpha(self.theme.text, 0.075).into()),
                text_color: if *selected {
                    self.theme.text
                } else {
                    alpha(self.theme.text, 0.75)
                },
                ..self.active(style)
            },
            Button::Reject => button::Appearance {
                background: alpha(self.theme.error, 0.1).into(),
                border_color: alpha(self.theme.error, 0.5),
                text_color: self.theme.error,
                ..appearance
            },
            Button::Text => button::Appearance {
                text_color: self.theme.text,
                ..self.active(style)
            },
            Button::ThemePreview(theme) => button::Appearance {
                background: Some(alpha(theme.text, 0.025).into()),
                border_color: alpha(theme.text, 0.5),
                text_color: theme.text,
                ..self.active(style)
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let appearance = button::Appearance {
            background: Some(alpha(self.theme.text, 0.015).into()),
            border_color: alpha(self.theme.text, 0.1),
            text_color: alpha(self.theme.text, 0.6),
            ..self.hovered(style)
        };

        match style {
            Button::Accept => button::Appearance {
                background: alpha(self.theme.target, 0.075).into(),
                border_color: alpha(self.theme.target, 0.4),
                text_color: alpha(self.theme.target, 0.85),
                ..appearance
            },
            Button::Basic => appearance,
            Button::Menu { selected } => button::Appearance {
                background: selected.then(|| alpha(self.theme.text, 0.07).into()),
                text_color: if *selected {
                    self.theme.text
                } else {
                    alpha(self.theme.text, 0.6)
                },
                ..self.active(style)
            },
            Button::Reject => button::Appearance {
                background: alpha(self.theme.error, 0.075).into(),
                border_color: alpha(self.theme.error, 0.4),
                text_color: alpha(self.theme.error, 0.85),
                ..appearance
            },
            Button::Text => button::Appearance {
                text_color: alpha(self.theme.text, 0.9),
                ..self.active(style)
            },
            Button::ThemePreview(theme) => button::Appearance {
                background: Some(alpha(theme.text, 0.015).into()),
                border_color: alpha(theme.text, 0.4),
                text_color: alpha(theme.text, 0.6),
                ..self.active(style)
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let appearance = button::Appearance {
            background: None,
            border_color: alpha(self.theme.text, 0.05),
            text_color: alpha(self.theme.text, 0.05),
            ..self.active(style)
        };

        match style {
            Button::Accept => button::Appearance {
                background: alpha(self.theme.target, 0.025).into(),
                border_color: alpha(self.theme.target, 0.05),
                text_color: alpha(self.theme.target, 0.05),
                ..appearance
            },
            Button::Basic => appearance,
            Button::Menu { selected } => button::Appearance {
                background: selected.then(|| alpha(self.theme.text, 0.025).into()),
                text_color: alpha(self.theme.text, 0.25),
                ..self.active(style)
            },
            Button::Reject => button::Appearance {
                background: alpha(self.theme.error, 0.025).into(),
                border_color: alpha(self.theme.error, 0.05),
                text_color: alpha(self.theme.error, 0.05),
                ..appearance
            },
            Button::Text => button::Appearance {
                text_color: alpha(self.theme.text, 0.25),
                ..self.active(style)
            },
            Button::ThemePreview(theme) => button::Appearance {
                background: None,
                border_color: alpha(theme.text, 0.05),
                text_color: alpha(theme.text, 0.05),
                ..self.active(style)
            },
        }
    }
}

#[derive(Default)]
pub enum Container {
    MenuSelected,
    ThemePreview {
        fg: Color,
        bg: Background,
    },
    #[default]
    Primary,
}

impl Container {
    pub fn theme_preview(theme: &data::Theme) -> Self {
        Container::ThemePreview {
            fg: theme.text,
            bg: theme.bg.into(),
        }
    }
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::MenuSelected => container::Appearance {
                text_color: Some(self.theme.text),
                background: Some(alpha(self.theme.text, 0.05).into()),
                ..Default::default()
            },
            Container::ThemePreview { fg, bg } => container::Appearance {
                text_color: Some(*fg),
                background: Some(*bg),
                ..Default::default()
            },
            Container::Primary => Default::default(),
        }
    }
}

#[derive(Default)]
pub enum Rule {
    #[default]
    Divider,
}

impl rule::StyleSheet for Theme {
    type Style = Rule;

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
        use iced::widget::rule::FillMode;

        match style {
            Rule::Divider => rule::Appearance {
                color: alpha(self.theme.hit, 0.05),
                width: 1,
                radius: 0.0,
                fill_mode: FillMode::Full,
            },
        }
    }
}

#[derive(Default)]
pub enum Scrollable {
    #[default]
    Divider,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: self.theme.text,
            scroller: scrollable::Scroller {
                color: self.theme.error,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: self.theme.error,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> scrollable::Scrollbar {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Scrollbar {
        self.active(style)
    }
}

#[derive(Default)]
pub enum TextInput {
    #[default]
    Default,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::TRANSPARENT.into(),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: alpha(self.theme.hit, 0.1),
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: alpha(self.theme.hit, 0.015).into(),
            border_color: alpha(self.theme.hit, 0.1),
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: alpha(self.theme.hit, 0.025).into(),
            border_color: alpha(self.theme.hit, 0.25),
            ..self.active(style)
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color {
            a: 0.25,
            ..self.theme.hit
        }
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.theme.text
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color {
            a: 0.05,
            ..self.theme.text
        }
    }
}

#[derive(Default, Clone)]
pub enum PickList {
    #[default]
    Default,
}

impl pick_list::StyleSheet for Theme {
    type Style = PickList;

    fn active(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.theme.text,
            placeholder_color: alpha(self.theme.hit, 0.25),
            handle_color: self.theme.text,
            background: self.theme.bg.into(),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: alpha(self.theme.text, 0.1),
        }
    }

    fn hovered(&self, style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            border_color: alpha(self.theme.text, 0.25),
            ..self.active(style)
        }
    }
}

impl iced::overlay::menu::StyleSheet for Theme {
    type Style = PickList;

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        menu::Appearance {
            text_color: alpha(self.theme.text, 0.5),
            background: self.theme.bg.into(),
            border_width: 1.0,
            border_radius: 0.0,
            border_color: alpha(self.theme.text, 0.25),
            selected_text_color: self.theme.text,
            selected_background: alpha(self.theme.text, 0.05).into(),
        }
    }
}
