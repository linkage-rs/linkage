use crate::data::{self, Theme};
use iced::button::{self, Button};
use iced::scrollable::{self, Scrollable};
use iced::{Column, Element};

#[derive(Debug)]
pub struct State {
    scroll_state: scrollable::State,
    profile_buttons: Vec<button::State>,
    new_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl State {
    pub fn new() -> Self {
        Self {
            scroll_state: scrollable::State::new(),
            profile_buttons: Vec::new(),
            new_button: button::State::new(),
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        Column::new().into()
    }
}
