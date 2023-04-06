use crate::data::Theme;
use crate::{style, Element};

use iced::widget::{Button, Column, Container, Row, Scrollable, Space, Text};
use iced::{Alignment, Length};

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

    pub fn view(&self) -> Element<Message> {
        let title = Container::new(Text::new("Theme").size(18)).padding(6);

        let buttons = Column::with_children(
            self.themes
                .iter()
                .enumerate()
                .map(|(i, th)| {
                    let mut content = Row::new()
                        .push(
                            Text::new("\u{25a0}")
                                .size(18)
                                .style(style::Text::Override(th.target)),
                        )
                        .push(
                            Text::new("\u{25a0}")
                                .size(18)
                                .style(style::Text::Override(th.miss)),
                        )
                        .push(
                            Text::new("\u{25a0}")
                                .size(18)
                                .style(style::Text::Override(th.error)),
                        )
                        .push(
                            Text::new(th.name)
                                .size(16)
                                .style(style::Text::Override(th.text)),
                        )
                        .spacing(5)
                        .align_items(Alignment::Center);

                    if self.active == th.name {
                        content = content
                            .push(Space::with_width(Length::Fill))
                            .push(Text::new("\u{25cf}").size(18));
                    }

                    Container::new(
                        Button::new(content)
                            .on_press(Message::ThemePressed(i))
                            .style(style::Button::ThemePreview(th.clone()))
                            .width(Length::Fill)
                            .padding([3, 7, 3, 7]),
                    )
                    .style(style::Container::theme_preview(th))
                    .width(Length::Fill)
                    .into()
                })
                .collect(),
        )
        .width(225)
        .spacing(7)
        .padding([0, 0, 0, 6]);

        Scrollable::new(
            Column::new()
                .push(title)
                .push(buttons)
                .width(Length::Fill)
                .spacing(20)
                .padding(10),
        )
        .height(Length::Fill)
        .into()
    }
}
