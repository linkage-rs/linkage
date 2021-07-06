use crate::data::profile;
use crate::data::Theme;
use crate::font;
use crate::style;

use iced::button::{self, Button};
use iced::{
    Align, Column, Command, Container, Element, Length, Row, Rule, Space, Subscription, Text,
    VerticalAlignment,
};

#[derive(Debug)]
pub struct State {
    back_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackButtonPressed,
}

pub enum Event {
    Exit,
}

impl State {
    pub fn new() -> Self {
        Self {
            back_button: button::State::new(),
        }
    }

    pub fn update(
        &mut self,
        profiles: &mut profile::List,
        message: Message,
    ) -> Option<(Command<Message>, Event)> {
        match message {
            Message::BackButtonPressed => Some((Command::none(), Event::Exit)),
            _ => None,
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let State { back_button } = self;

        let back_button = Button::new(back_button, Text::new("\u{2190} Back to Training"))
            .style(style::button::text(theme))
            .on_press(Message::BackButtonPressed);

        Column::new().push(back_button).into()
    }
}
