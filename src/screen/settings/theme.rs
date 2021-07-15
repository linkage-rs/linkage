use crate::data::Theme;
use crate::style;
use iced::button::{self, Button};
use iced::scrollable::{self, Scrollable};
use iced::{Align, Column, Container, Element, Length, Row, Space, Text};

#[derive(Debug)]
pub struct State {
    buttons: Vec<button::State>,
    scroll: scrollable::State,
    themes: Vec<Theme>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemePressed(usize),
}

pub enum Event {
    SelectTheme(Theme),
}

impl State {
    pub fn new() -> Self {
        let themes = Theme::all();
        let buttons = vec![button::State::new(); themes.len()];

        Self {
            buttons,
            scroll: scrollable::State::new(),
            themes,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::ThemePressed(index) => {
                if let Some(theme) = self.themes.get(index) {
                    return Some(Event::SelectTheme(theme.clone()));
                }
            }
        }
        None
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let title = Container::new(Text::new("Theme").size(18)).padding(6);

        let buttons = Column::with_children(
            self.themes
                .iter()
                .enumerate()
                .zip(self.buttons.iter_mut())
                .map(|((i, th), state)| {
                    let mut text = Row::new()
                        .push(Text::new("\u{25a0}").size(18).color(th.target))
                        .push(Text::new("\u{25a0}").size(18).color(th.miss))
                        .push(Text::new("\u{25a0}").size(18).color(th.error))
                        .push(Text::new(th.name.clone()).size(16))
                        .spacing(5)
                        .align_items(Align::Center);

                    if theme.name == th.name {
                        text = text
                            .push(Space::with_width(Length::Fill))
                            .push(Text::new("\u{25cf}").size(18).color(th.text));
                    }

                    Container::new(
                        Button::new(state, text)
                            .on_press(Message::ThemePressed(i))
                            .style(style::button::basic(th))
                            .width(Length::Fill)
                            .padding([3, 7, 3, 7]),
                    )
                    .style(style::container::primary(th))
                    .width(Length::Fill)
                    .into()
                })
                .collect(),
        )
        .width(Length::Units(175))
        .spacing(7)
        .padding([0, 0, 0, 6]);

        Scrollable::new(&mut self.scroll)
            .push(title)
            .push(buttons)
            .height(Length::Fill)
            .width(Length::Fill)
            .spacing(20)
            .padding(10)
            .into()
    }
}
