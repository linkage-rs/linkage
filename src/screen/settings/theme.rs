use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length, Padding};

use crate::data::Theme;

#[derive(Debug)]
pub struct State {
    themes: Vec<Theme>,
    active: &'static str,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemePressed(usize),
}

pub enum Event {
    SelectTheme(Theme),
}

impl State {
    pub fn new(active: &'static str) -> Self {
        let themes = Theme::all();

        Self { themes, active }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::ThemePressed(index) => {
                if let Some(theme) = self.themes.get(index) {
                    self.active = theme.name;
                    return Some(Event::SelectTheme(theme.clone()));
                }
            }
        }
        None
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = container(text("Theme").size(18)).padding(6);

        let buttons = column(
            self.themes
                .iter()
                .enumerate()
                .map(|(i, th)| {
                    let target_color = th.target;
                    let miss_color = th.miss;
                    let error_color = th.error;
                    let text_color = th.text;
                    let bg_color = th.bg;

                    let mut content = row![
                        text("\u{25a0}").size(18).color(target_color),
                        text("\u{25a0}").size(18).color(miss_color),
                        text("\u{25a0}").size(18).color(error_color),
                        text(th.name).size(16).color(text_color),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center);

                    if self.active == th.name {
                        content = content
                            .push(Space::new().width(Length::Fill))
                            .push(text("\u{25cf}").size(18));
                    }

                    let bg = bg_color;
                    container(
                        button(content)
                            .on_press(Message::ThemePressed(i))
                            .style(move |_theme: &iced::Theme, status| {
                                let base = iced::widget::button::Style {
                                    border: iced::Border {
                                        radius: 2.0.into(),
                                        width: 1.0,
                                        color: iced::Color {
                                            a: 0.15,
                                            ..text_color
                                        },
                                    },
                                    text_color: iced::Color {
                                        a: 0.75,
                                        ..text_color
                                    },
                                    ..iced::widget::button::Style::default()
                                };
                                match status {
                                    iced::widget::button::Status::Active => base,
                                    iced::widget::button::Status::Hovered => {
                                        iced::widget::button::Style {
                                            background: Some(
                                                iced::Color {
                                                    a: 0.025,
                                                    ..text_color
                                                }
                                                .into(),
                                            ),
                                            border: iced::Border {
                                                color: iced::Color {
                                                    a: 0.5,
                                                    ..text_color
                                                },
                                                ..base.border
                                            },
                                            text_color: text_color,
                                            ..base
                                        }
                                    }
                                    iced::widget::button::Status::Pressed => {
                                        iced::widget::button::Style {
                                            background: Some(
                                                iced::Color {
                                                    a: 0.015,
                                                    ..text_color
                                                }
                                                .into(),
                                            ),
                                            border: iced::Border {
                                                color: iced::Color {
                                                    a: 0.4,
                                                    ..text_color
                                                },
                                                ..base.border
                                            },
                                            text_color: iced::Color {
                                                a: 0.6,
                                                ..text_color
                                            },
                                            ..base
                                        }
                                    }
                                    iced::widget::button::Status::Disabled => base,
                                }
                            })
                            .width(Length::Fill)
                            .padding(Padding::ZERO.top(3.0).right(7.0).bottom(3.0).left(7.0)),
                    )
                    .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                        text_color: Some(text_color),
                        background: Some(bg.into()),
                        ..Default::default()
                    })
                    .width(Length::Fill)
                    .into()
                })
                .collect::<Vec<Element<Message>>>(),
        )
        .width(225)
        .spacing(7)
        .padding(Padding::ZERO.left(6.0));

        scrollable(
            column![title, buttons]
                .width(Length::Fill)
                .spacing(20)
                .padding(10),
        )
        .height(Length::Fill)
        .into()
    }
}
