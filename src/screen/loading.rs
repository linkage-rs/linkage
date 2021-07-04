use crate::data::user;
use crate::data::Theme;

use iced::{Element, Text};

#[derive(Debug)]
pub struct Loading {}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<user::Saved, Error>),
}

pub enum Event {
    Load(user::List),
}

pub async fn load() -> Message {
    Message::Loaded(Ok(user::Saved::default()))
}

impl Loading {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self, _theme: &Theme) -> Element<Message> {
        Text::new("Loading").into()
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Loaded(result) => {
                Some(Event::Load(result.unwrap_or(user::Saved::default()).into()))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Corrupted,
    FileSystem,
}
