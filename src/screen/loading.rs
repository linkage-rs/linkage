use crate::data::profile;
use crate::data::Theme;

use iced::{Element, Text};

#[derive(Debug)]
pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(profile::Saved),
}

pub enum Event {
    Load(profile::List),
}

pub async fn load() -> Message {
    let loaded = profile::Saved::load().await;
    let saved = loaded.unwrap_or_else(|_| profile::Saved::default());
    Message::Loaded(saved)
}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self, _theme: &Theme) -> Element<Message> {
        Text::new("Loading").into()
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Loaded(saved) => Some(Event::Load(saved.into())),
        }
    }
}
