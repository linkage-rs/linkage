use crate::data::profile;
use crate::data::Theme;

use iced::{Element, Text};

#[derive(Debug)]
pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<profile::Saved, Error>),
}

pub enum Event {
    Load(profile::List),
}

pub async fn load() -> Message {
    Message::Loaded(Ok(profile::Saved::default()))
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
            Message::Loaded(result) => Some(Event::Load(
                result.unwrap_or(profile::Saved::default()).into(),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Corrupted,
    FileSystem,
}
