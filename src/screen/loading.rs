use crate::data::User;

use iced::{Element, Text};

#[derive(Debug)]
pub struct Loading {}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<User, Error>),
}

pub enum Event {
    Load(User),
}

pub async fn load() -> Message {
    Message::Loaded(Ok(User::default()))
}

impl Loading {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self) -> Element<Message> {
        Text::new("Loading").into()
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Loaded(result) => Some(Event::Load(result.unwrap_or(User::default()))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Corrupted,
    FileSystem,
}
