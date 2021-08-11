use crate::data;
use crate::data::profile;
use crate::data::Theme;

use iced::{Element, Text};

#[derive(Debug)]
pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(data::Saved),
}

pub enum Event {
    Load {
        profiles: profile::List,
        theme: Theme,
    },
}

pub async fn load() -> Message {
    let saved = data::Saved::load().await.unwrap_or_default();
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
            Message::Loaded(saved) => {
                let data::Saved {
                    profiles,
                    theme_name,
                    ..
                } = saved;
                Some(Event::Load {
                    profiles: profiles.into(),
                    theme: Theme::from_name(&theme_name).unwrap_or_default(),
                })
            }
        }
    }
}
