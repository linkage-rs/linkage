use iced::widget::{
    button, container, overlay, pick_list, rule, scrollable, text, text_input, toggler,
};
use iced::{Background, Border, Color};

use crate::data::Theme;

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

impl Text {
    /// Produce a text style function for the given variant and theme.
    pub fn style(self, theme: &Theme) -> text::Style {
        let color = match self {
            Text::Default => None,
            Text::Error => Some(theme.error()),
            Text::Metric(metric) => Some(theme.metric(metric)),
            Text::Miss => Some(theme.miss()),
            Text::Override(color) => Some(color),
            Text::Target => Some(theme.target()),
        };
        text::Style { color }
    }
}

impl text::Catalog for Theme {
    type Class<'a> = Text;

    fn default<'a>() -> Self::Class<'a> {
        Text::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        class.style(&self)
    }
}

#[derive(Default, Clone)]
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
    ThemePreview(Theme),
}

impl Button {
    pub fn style(&self, theme: &Theme, status: button::Status) -> button::Style {
        let base = button::Style {
            border: Border {
                radius: 2.0.into(),
                width: 1.0,
                color: alpha(theme.text(), 0.15),
            },
            text_color: alpha(theme.text(), 0.75),
            ..button::Style::default()
        };

        match &self {
            Button::Accept => match status {
                button::Status::Active => button::Style {
                    background: Some(alpha(theme.target(), 0.05).into()),
                    border: Border {
                        color: alpha(theme.target(), 0.15),
                        ..base.border
                    },
                    text_color: alpha(theme.target(), 0.75),
                    ..base
                },
                button::Status::Hovered => button::Style {
                    background: Some(alpha(theme.target(), 0.1).into()),
                    border: Border {
                        color: alpha(theme.target(), 0.5),
                        ..base.border
                    },
                    text_color: theme.target(),
                    ..base
                },
                button::Status::Pressed => button::Style {
                    background: Some(alpha(theme.target(), 0.075).into()),
                    border: Border {
                        color: alpha(theme.target(), 0.4),
                        ..base.border
                    },
                    text_color: alpha(theme.target(), 0.85),
                    ..base
                },
                button::Status::Disabled => button::Style {
                    background: Some(alpha(theme.target(), 0.025).into()),
                    border: Border {
                        color: alpha(theme.target(), 0.05),
                        ..base.border
                    },
                    text_color: alpha(theme.target(), 0.05),
                    ..base
                },
            },
            Button::Basic => match status {
                button::Status::Active => base,
                button::Status::Hovered => button::Style {
                    background: Some(alpha(theme.text(), 0.025).into()),
                    border: Border {
                        color: alpha(theme.text(), 0.5),
                        ..base.border
                    },
                    text_color: theme.text(),
                    ..base
                },
                button::Status::Pressed => button::Style {
                    background: Some(alpha(theme.text(), 0.015).into()),
                    border: Border {
                        color: alpha(theme.text(), 0.1),
                        ..base.border
                    },
                    text_color: alpha(theme.text(), 0.6),
                    ..base
                },
                button::Status::Disabled => button::Style {
                    background: None,
                    border: Border {
                        color: alpha(theme.text(), 0.05),
                        ..base.border
                    },
                    text_color: alpha(theme.text(), 0.05),
                    ..base
                },
            },
            Button::Menu { selected } => {
                let selected = *selected;
                match status {
                    button::Status::Active => button::Style {
                        background: if selected {
                            Some(alpha(theme.text(), 0.05).into())
                        } else {
                            None
                        },
                        text_color: if selected {
                            theme.text()
                        } else {
                            alpha(theme.text(), 0.5)
                        },
                        ..button::Style::default()
                    },
                    button::Status::Hovered => button::Style {
                        background: if selected {
                            Some(alpha(theme.text(), 0.075).into())
                        } else {
                            None
                        },
                        text_color: if selected {
                            theme.text()
                        } else {
                            alpha(theme.text(), 0.75)
                        },
                        ..button::Style::default()
                    },
                    button::Status::Pressed => button::Style {
                        background: if selected {
                            Some(alpha(theme.text(), 0.07).into())
                        } else {
                            None
                        },
                        text_color: if selected {
                            theme.text()
                        } else {
                            alpha(theme.text(), 0.6)
                        },
                        ..button::Style::default()
                    },
                    button::Status::Disabled => button::Style {
                        background: if selected {
                            Some(alpha(theme.text(), 0.025).into())
                        } else {
                            None
                        },
                        text_color: alpha(theme.text(), 0.25),
                        ..button::Style::default()
                    },
                }
            }
            Button::Reject => match status {
                button::Status::Active => button::Style {
                    background: Some(alpha(theme.error(), 0.05).into()),
                    border: Border {
                        color: alpha(theme.error(), 0.15),
                        ..base.border
                    },
                    text_color: alpha(theme.error(), 0.75),
                    ..base
                },
                button::Status::Hovered => button::Style {
                    background: Some(alpha(theme.error(), 0.1).into()),
                    border: Border {
                        color: alpha(theme.error(), 0.5),
                        ..base.border
                    },
                    text_color: theme.error(),
                    ..base
                },
                button::Status::Pressed => button::Style {
                    background: Some(alpha(theme.error(), 0.075).into()),
                    border: Border {
                        color: alpha(theme.error(), 0.4),
                        ..base.border
                    },
                    text_color: alpha(theme.error(), 0.85),
                    ..base
                },
                button::Status::Disabled => button::Style {
                    background: Some(alpha(theme.error(), 0.025).into()),
                    border: Border {
                        color: alpha(theme.error(), 0.05),
                        ..base.border
                    },
                    text_color: alpha(theme.error(), 0.05),
                    ..base
                },
            },
            Button::Text => match status {
                button::Status::Active => button::Style {
                    text_color: alpha(theme.text(), 0.5),
                    border: Border {
                        width: 0.0,
                        ..base.border
                    },
                    ..base
                },
                button::Status::Hovered => button::Style {
                    text_color: theme.text(),
                    border: Border {
                        width: 0.0,
                        ..base.border
                    },
                    ..base
                },
                button::Status::Pressed => button::Style {
                    text_color: alpha(theme.text(), 0.9),
                    border: Border {
                        width: 0.0,
                        ..base.border
                    },
                    ..base
                },
                button::Status::Disabled => button::Style {
                    text_color: alpha(theme.text(), 0.25),
                    border: Border {
                        width: 0.0,
                        ..base.border
                    },
                    ..base
                },
            },
            Button::ThemePreview(theme) => match status {
                button::Status::Active => button::Style {
                    border: Border {
                        color: alpha(theme.text(), 0.15),
                        ..base.border
                    },
                    text_color: alpha(theme.text(), 0.75),
                    ..base
                },
                button::Status::Hovered => button::Style {
                    background: Some(alpha(theme.text(), 0.025).into()),
                    border: Border {
                        color: alpha(theme.text(), 0.5),
                        ..base.border
                    },
                    text_color: theme.text(),
                    ..base
                },
                button::Status::Pressed => button::Style {
                    background: Some(alpha(theme.text(), 0.015).into()),
                    border: Border {
                        color: alpha(theme.text(), 0.4),
                        ..base.border
                    },
                    text_color: alpha(theme.text(), 0.6),
                    ..base
                },
                button::Status::Disabled => button::Style {
                    background: None,
                    border: Border {
                        color: alpha(theme.text(), 0.05),
                        ..base.border
                    },
                    text_color: alpha(theme.text(), 0.05),
                    ..base
                },
            },
        }
    }
}

impl button::Catalog for Theme {
    type Class<'a> = Button;

    fn default<'a>() -> Self::Class<'a> {
        Button::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        class.style(&self, status)
    }
}

#[derive(Default, Clone)]
pub enum Container {
    MenuSelected,
    RoundedBox,
    ThemePreview {
        fg: Color,
        bg: Background,
    },
    Toast,
    #[default]
    Primary,
}

impl Container {
    pub fn theme_preview(theme: &Theme) -> Self {
        Container::ThemePreview {
            fg: theme.text(),
            bg: theme.background().into(),
        }
    }

    pub fn style(&self, theme: &Theme) -> container::Style {
        match self {
            Container::MenuSelected => container::Style {
                text_color: Some(theme.text()),
                background: Some(alpha(theme.text(), 0.05).into()),
                ..Default::default()
            },
            Container::RoundedBox => {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(palette.background.weak.color.into()),
                    text_color: Some(palette.background.weak.text),
                    border: iced::border::rounded(2),
                    ..Default::default()
                }
            }
            Container::ThemePreview { fg, bg } => container::Style {
                text_color: Some(*fg),
                background: Some(*bg),
                ..Default::default()
            },
            Container::Toast => {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(alpha(palette.background.strong.color, 0.92).into()),
                    text_color: Some(theme.error()),
                    border: Border {
                        radius: 4.0.into(),
                        width: 1.0,
                        color: alpha(theme.error(), 0.3),
                    },
                    ..Default::default()
                }
            }
            Container::Primary => container::Style::default(),
        }
    }
}

impl container::Catalog for Theme {
    type Class<'a> = Container;

    fn default<'a>() -> Self::Class<'a> {
        Container::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        class.style(&self)
    }
}

#[derive(Default, Clone)]
pub enum Rule {
    #[default]
    Divider,
}

impl Rule {
    pub fn style(&self, theme: &Theme) -> rule::Style {
        match &self {
            Rule::Divider => rule::Style {
                color: alpha(theme.hit(), 0.05),
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            },
        }
    }
}

impl rule::Catalog for Theme {
    type Class<'a> = Rule;

    fn default<'a>() -> Self::Class<'a> {
        Rule::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> rule::Style {
        class.style(&self)
    }
}

#[derive(Clone)]
pub struct Scrollable;

impl Scrollable {
    pub fn style(&self, theme: &Theme, _status: scrollable::Status) -> scrollable::Style {
        let rail = scrollable::Rail {
            background: None,
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: theme.text(),
            },
            scroller: scrollable::Scroller {
                background: match _status {
                    scrollable::Status::Active { .. } => {
                        theme.extended_palette().background.neutral.color.into()
                    }
                    scrollable::Status::Hovered { .. } => {
                        theme.extended_palette().background.strongest.color.into()
                    }
                    scrollable::Status::Dragged { .. } => {
                        theme.extended_palette().background.stronger.color.into()
                    }
                },
                border: Border {
                    radius: 2.0.into(),
                    width: 0.0,
                    color: theme.error(),
                },
            },
        };
        scrollable::Style {
            container: container::Style::default(),
            vertical_rail: rail.clone(),
            horizontal_rail: rail,
            gap: None,
            auto_scroll: scrollable::AutoScroll {
                background: Background::Color(Color::TRANSPARENT),
                border: Border::default(),
                shadow: iced::Shadow::default(),
                icon: theme.text(),
            },
        }
    }
}

impl scrollable::Catalog for Theme {
    type Class<'a> = Scrollable;

    fn default<'a>() -> Self::Class<'a> {
        Scrollable
    }

    fn style(&self, class: &Self::Class<'_>, status: scrollable::Status) -> scrollable::Style {
        class.style(&self, status)
    }
}

#[derive(Clone)]
pub struct TextInput;

impl TextInput {
    pub fn style(&self, theme: &Theme, status: text_input::Status) -> text_input::Style {
        let active = text_input::Style {
            background: Color::TRANSPARENT.into(),
            border: Border {
                radius: 2.0.into(),
                width: 1.0,
                color: alpha(theme.hit(), 0.1),
            },
            icon: theme.text(),
            placeholder: alpha(theme.hit(), 0.25),
            value: theme.text(),
            selection: alpha(theme.text(), 0.05),
        };

        match status {
            text_input::Status::Active => active,
            text_input::Status::Hovered => text_input::Style {
                background: alpha(theme.hit(), 0.015).into(),
                border: Border {
                    color: alpha(theme.hit(), 0.1),
                    ..active.border
                },
                ..active
            },
            text_input::Status::Focused { .. } => text_input::Style {
                background: alpha(theme.hit(), 0.025).into(),
                border: Border {
                    color: alpha(theme.hit(), 0.25),
                    ..active.border
                },
                ..active
            },
            text_input::Status::Disabled => active,
        }
    }
}

impl text_input::Catalog for Theme {
    type Class<'a> = TextInput;

    fn default<'a>() -> Self::Class<'a> {
        TextInput
    }

    fn style(&self, class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        class.style(&self, status)
    }
}

#[derive(Clone)]
pub struct PickList;

impl PickList {
    pub fn style(&self, theme: &Theme, status: pick_list::Status) -> pick_list::Style {
        let theme = theme.clone();

        let active = pick_list::Style {
            text_color: theme.text(),
            placeholder_color: alpha(theme.hit(), 0.25),
            handle_color: theme.text(),
            background: theme.background().into(),
            border: Border {
                radius: 2.0.into(),
                width: 1.0,
                color: alpha(theme.text(), 0.1),
            },
        };

        match status {
            pick_list::Status::Active => active,
            pick_list::Status::Hovered => pick_list::Style {
                border: Border {
                    color: alpha(theme.text(), 0.25),
                    ..active.border
                },
                ..active
            },
            pick_list::Status::Opened { .. } => pick_list::Style {
                border: Border {
                    color: alpha(theme.text(), 0.25),
                    ..active.border
                },
                ..active
            },
        }
    }
}

impl pick_list::Catalog for Theme {
    type Class<'a> = PickList;

    fn default<'a>() -> <Self as pick_list::Catalog>::Class<'a> {
        PickList
    }

    fn style(
        &self,
        class: &<Self as pick_list::Catalog>::Class<'_>,
        status: pick_list::Status,
    ) -> pick_list::Style {
        class.style(&self, status)
    }
}

#[derive(Default, Clone)]
pub enum Overlay {
    #[default]
    Default,
}

impl Overlay {
    fn style(&self, theme: &Theme) -> overlay::menu::Style {
        let palette = theme.extended_palette();
        overlay::menu::Style {
            background: palette.background.strong.color.into(),
            border: Border {
                color: palette.background.weak.text,
                width: 1.0,
                radius: 2.0.into(),
            },
            text_color: palette.background.strong.text,
            selected_text_color: palette.background.stronger.text,
            selected_background: palette.background.stronger.color.into(),
            shadow: iced::Shadow {
                color: palette.background.weak.color,
                offset: iced::Vector::new(0.0, 8.0),
                blur_radius: 12.0,
            },
        }
    }
}

impl overlay::menu::Catalog for Theme {
    type Class<'a> = Overlay;

    fn default<'a>() -> <Self as iced::overlay::menu::Catalog>::Class<'a> {
        Overlay::default()
    }

    fn style(
        &self,
        class: &<Self as iced::overlay::menu::Catalog>::Class<'_>,
    ) -> iced::overlay::menu::Style {
        class.style(&self)
    }
}

#[derive(Clone)]
pub struct Toggler;

impl Toggler {
    pub fn style(&self, theme: &Theme, status: toggler::Status) -> toggler::Style {
        let palette = theme.palette();
        let extended = theme.extended_palette();

        match status {
            toggler::Status::Active { is_toggled } => toggler::Style {
                background: if is_toggled {
                    alpha(palette.success, 0.6).into()
                } else {
                    alpha(extended.background.strong.color, 0.8).into()
                },
                background_border_width: 1.0,
                background_border_color: if is_toggled {
                    alpha(palette.success, 0.3)
                } else {
                    alpha(palette.text, 0.15)
                },
                foreground: if is_toggled {
                    palette.background.into()
                } else {
                    alpha(palette.text, 0.5).into()
                },
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
                text_color: None,
                border_radius: None,
                padding_ratio: 0.25,
            },
            toggler::Status::Hovered { is_toggled } => toggler::Style {
                background: if is_toggled {
                    alpha(palette.success, 0.75).into()
                } else {
                    alpha(extended.background.strong.color, 0.9).into()
                },
                background_border_width: 1.0,
                background_border_color: if is_toggled {
                    alpha(palette.success, 0.5)
                } else {
                    alpha(palette.text, 0.25)
                },
                foreground: if is_toggled {
                    palette.background.into()
                } else {
                    alpha(palette.text, 0.7).into()
                },
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
                text_color: None,
                border_radius: None,
                padding_ratio: 0.25,
            },
            toggler::Status::Disabled { .. } => toggler::Style {
                background: alpha(extended.background.weak.color, 0.5).into(),
                background_border_width: 1.0,
                background_border_color: alpha(palette.text, 0.05),
                foreground: alpha(palette.text, 0.15).into(),
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
                text_color: None,
                border_radius: None,
                padding_ratio: 0.25,
            },
        }
    }
}

impl toggler::Catalog for Theme {
    type Class<'a> = Toggler;

    fn default<'a>() -> Self::Class<'a> {
        Toggler
    }

    fn style(&self, class: &Self::Class<'_>, status: toggler::Status) -> toggler::Style {
        class.style(self, status)
    }
}

fn alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}
