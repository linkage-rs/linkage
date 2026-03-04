use iced::widget::{button, container, pick_list, rule, scrollable, text, text_input};
use iced::{Background, Border, Color};

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

    pub fn data(&self) -> &data::Theme {
        &self.theme
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

impl Text {
    /// Produce a text style function for the given variant and theme.
    pub fn style_fn(self, theme: &data::Theme) -> text::Style {
        let color = match self {
            Text::Default => None,
            Text::Error => Some(theme.error),
            Text::Metric(metric) => Some(theme.metric(metric)),
            Text::Miss => Some(theme.miss),
            Text::Override(color) => Some(color),
            Text::Target => Some(theme.target),
        };
        text::Style { color }
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
    ThemePreview(data::Theme),
}

impl Button {
    pub fn style_fn<'a>(
        self,
        theme: &data::Theme,
    ) -> impl Fn(&iced::Theme, button::Status) -> button::Style + 'a {
        let t = theme.clone();
        move |_iced_theme: &iced::Theme, status: button::Status| {
            let base = button::Style {
                border: Border {
                    radius: 2.0.into(),
                    width: 1.0,
                    color: alpha(t.text, 0.15),
                },
                text_color: alpha(t.text, 0.75),
                ..button::Style::default()
            };

            match &self {
                Button::Accept => match status {
                    button::Status::Active => button::Style {
                        background: Some(alpha(t.target, 0.05).into()),
                        border: Border {
                            color: alpha(t.target, 0.15),
                            ..base.border
                        },
                        text_color: alpha(t.target, 0.75),
                        ..base
                    },
                    button::Status::Hovered => button::Style {
                        background: Some(alpha(t.target, 0.1).into()),
                        border: Border {
                            color: alpha(t.target, 0.5),
                            ..base.border
                        },
                        text_color: t.target,
                        ..base
                    },
                    button::Status::Pressed => button::Style {
                        background: Some(alpha(t.target, 0.075).into()),
                        border: Border {
                            color: alpha(t.target, 0.4),
                            ..base.border
                        },
                        text_color: alpha(t.target, 0.85),
                        ..base
                    },
                    button::Status::Disabled => button::Style {
                        background: Some(alpha(t.target, 0.025).into()),
                        border: Border {
                            color: alpha(t.target, 0.05),
                            ..base.border
                        },
                        text_color: alpha(t.target, 0.05),
                        ..base
                    },
                },
                Button::Basic => match status {
                    button::Status::Active => base,
                    button::Status::Hovered => button::Style {
                        background: Some(alpha(t.text, 0.025).into()),
                        border: Border {
                            color: alpha(t.text, 0.5),
                            ..base.border
                        },
                        text_color: t.text,
                        ..base
                    },
                    button::Status::Pressed => button::Style {
                        background: Some(alpha(t.text, 0.015).into()),
                        border: Border {
                            color: alpha(t.text, 0.1),
                            ..base.border
                        },
                        text_color: alpha(t.text, 0.6),
                        ..base
                    },
                    button::Status::Disabled => button::Style {
                        background: None,
                        border: Border {
                            color: alpha(t.text, 0.05),
                            ..base.border
                        },
                        text_color: alpha(t.text, 0.05),
                        ..base
                    },
                },
                Button::Menu { selected } => {
                    let selected = *selected;
                    match status {
                        button::Status::Active => button::Style {
                            background: if selected {
                                Some(alpha(t.text, 0.05).into())
                            } else {
                                None
                            },
                            text_color: if selected { t.text } else { alpha(t.text, 0.5) },
                            ..button::Style::default()
                        },
                        button::Status::Hovered => button::Style {
                            background: if selected {
                                Some(alpha(t.text, 0.075).into())
                            } else {
                                None
                            },
                            text_color: if selected {
                                t.text
                            } else {
                                alpha(t.text, 0.75)
                            },
                            ..button::Style::default()
                        },
                        button::Status::Pressed => button::Style {
                            background: if selected {
                                Some(alpha(t.text, 0.07).into())
                            } else {
                                None
                            },
                            text_color: if selected { t.text } else { alpha(t.text, 0.6) },
                            ..button::Style::default()
                        },
                        button::Status::Disabled => button::Style {
                            background: if selected {
                                Some(alpha(t.text, 0.025).into())
                            } else {
                                None
                            },
                            text_color: alpha(t.text, 0.25),
                            ..button::Style::default()
                        },
                    }
                }
                Button::Reject => match status {
                    button::Status::Active => button::Style {
                        background: Some(alpha(t.error, 0.05).into()),
                        border: Border {
                            color: alpha(t.error, 0.15),
                            ..base.border
                        },
                        text_color: alpha(t.error, 0.75),
                        ..base
                    },
                    button::Status::Hovered => button::Style {
                        background: Some(alpha(t.error, 0.1).into()),
                        border: Border {
                            color: alpha(t.error, 0.5),
                            ..base.border
                        },
                        text_color: t.error,
                        ..base
                    },
                    button::Status::Pressed => button::Style {
                        background: Some(alpha(t.error, 0.075).into()),
                        border: Border {
                            color: alpha(t.error, 0.4),
                            ..base.border
                        },
                        text_color: alpha(t.error, 0.85),
                        ..base
                    },
                    button::Status::Disabled => button::Style {
                        background: Some(alpha(t.error, 0.025).into()),
                        border: Border {
                            color: alpha(t.error, 0.05),
                            ..base.border
                        },
                        text_color: alpha(t.error, 0.05),
                        ..base
                    },
                },
                Button::Text => match status {
                    button::Status::Active => button::Style {
                        text_color: alpha(t.text, 0.5),
                        border: Border {
                            width: 0.0,
                            ..base.border
                        },
                        ..base
                    },
                    button::Status::Hovered => button::Style {
                        text_color: t.text,
                        border: Border {
                            width: 0.0,
                            ..base.border
                        },
                        ..base
                    },
                    button::Status::Pressed => button::Style {
                        text_color: alpha(t.text, 0.9),
                        border: Border {
                            width: 0.0,
                            ..base.border
                        },
                        ..base
                    },
                    button::Status::Disabled => button::Style {
                        text_color: alpha(t.text, 0.25),
                        border: Border {
                            width: 0.0,
                            ..base.border
                        },
                        ..base
                    },
                },
                Button::ThemePreview(theme_preview) => {
                    let tp = theme_preview.clone();
                    match status {
                        button::Status::Active => button::Style {
                            border: Border {
                                color: alpha(tp.text, 0.15),
                                ..base.border
                            },
                            text_color: alpha(tp.text, 0.75),
                            ..base
                        },
                        button::Status::Hovered => button::Style {
                            background: Some(alpha(tp.text, 0.025).into()),
                            border: Border {
                                color: alpha(tp.text, 0.5),
                                ..base.border
                            },
                            text_color: tp.text,
                            ..base
                        },
                        button::Status::Pressed => button::Style {
                            background: Some(alpha(tp.text, 0.015).into()),
                            border: Border {
                                color: alpha(tp.text, 0.4),
                                ..base.border
                            },
                            text_color: alpha(tp.text, 0.6),
                            ..base
                        },
                        button::Status::Disabled => button::Style {
                            background: None,
                            border: Border {
                                color: alpha(tp.text, 0.05),
                                ..base.border
                            },
                            text_color: alpha(tp.text, 0.05),
                            ..base
                        },
                    }
                }
            }
        }
    }
}

#[derive(Default, Clone)]
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

    pub fn style_fn<'a>(
        self,
        theme: &data::Theme,
    ) -> impl Fn(&iced::Theme) -> container::Style + 'a {
        let t = theme.clone();
        move |_iced_theme: &iced::Theme| match &self {
            Container::MenuSelected => container::Style {
                text_color: Some(t.text),
                background: Some(alpha(t.text, 0.05).into()),
                ..Default::default()
            },
            Container::ThemePreview { fg, bg } => container::Style {
                text_color: Some(*fg),
                background: Some(*bg),
                ..Default::default()
            },
            Container::Primary => container::Style::default(),
        }
    }
}

#[derive(Default, Clone)]
pub enum Rule {
    #[default]
    Divider,
}

impl Rule {
    pub fn style_fn<'a>(self, theme: &data::Theme) -> impl Fn(&iced::Theme) -> rule::Style + 'a {
        let t = theme.clone();
        move |_iced_theme: &iced::Theme| match &self {
            Rule::Divider => rule::Style {
                color: alpha(t.hit, 0.05),
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            },
        }
    }
}

#[derive(Default, Clone)]
pub enum Scrollable {
    #[default]
    Default,
}

impl Scrollable {
    pub fn style_fn<'a>(
        self,
        theme: &data::Theme,
    ) -> impl Fn(&iced::Theme, scrollable::Status) -> scrollable::Style + 'a {
        let t = theme.clone();
        move |_iced_theme: &iced::Theme, _status: scrollable::Status| {
            let rail = scrollable::Rail {
                background: None,
                border: Border {
                    radius: 0.0.into(),
                    width: 0.0,
                    color: t.text,
                },
                scroller: scrollable::Scroller {
                    background: t.error.into(),
                    border: Border {
                        radius: 2.0.into(),
                        width: 0.0,
                        color: t.error,
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
                    icon: t.text,
                },
            }
        }
    }
}

#[derive(Default, Clone)]
pub enum TextInput {
    #[default]
    Default,
}

impl TextInput {
    pub fn style_fn<'a>(
        self,
        theme: &data::Theme,
    ) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style + 'a {
        let t = theme.clone();
        move |_iced_theme: &iced::Theme, status: text_input::Status| {
            let active = text_input::Style {
                background: Color::TRANSPARENT.into(),
                border: Border {
                    radius: 0.0.into(),
                    width: 1.0,
                    color: alpha(t.hit, 0.1),
                },
                icon: t.text,
                placeholder: Color { a: 0.25, ..t.hit },
                value: t.text,
                selection: Color { a: 0.05, ..t.text },
            };

            match status {
                text_input::Status::Active => active,
                text_input::Status::Hovered => text_input::Style {
                    background: alpha(t.hit, 0.015).into(),
                    border: Border {
                        color: alpha(t.hit, 0.1),
                        ..active.border
                    },
                    ..active
                },
                text_input::Status::Focused { .. } => text_input::Style {
                    background: alpha(t.hit, 0.025).into(),
                    border: Border {
                        color: alpha(t.hit, 0.25),
                        ..active.border
                    },
                    ..active
                },
                text_input::Status::Disabled => active,
            }
        }
    }
}

#[derive(Default, Clone)]
pub enum PickList {
    #[default]
    Default,
}

impl PickList {
    pub fn style_fn<'a>(
        self,
        theme: &data::Theme,
    ) -> impl Fn(&iced::Theme, pick_list::Status) -> pick_list::Style + 'a {
        let t = theme.clone();
        move |_iced_theme: &iced::Theme, status: pick_list::Status| {
            let active = pick_list::Style {
                text_color: t.text,
                placeholder_color: alpha(t.hit, 0.25),
                handle_color: t.text,
                background: t.bg.into(),
                border: Border {
                    radius: 0.0.into(),
                    width: 1.0,
                    color: alpha(t.text, 0.1),
                },
            };

            match status {
                pick_list::Status::Active => active,
                pick_list::Status::Hovered => pick_list::Style {
                    border: Border {
                        color: alpha(t.text, 0.25),
                        ..active.border
                    },
                    ..active
                },
                pick_list::Status::Opened { .. } => pick_list::Style {
                    border: Border {
                        color: alpha(t.text, 0.25),
                        ..active.border
                    },
                    ..active
                },
            }
        }
    }
}
