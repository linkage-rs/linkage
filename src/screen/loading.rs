use iced::widget::text;

use crate::data::{Theme, profile};
use crate::{Element, data};

#[derive(Debug, Default)]
pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<data::Saved, String>),
}

pub enum Event {
    Load {
        profiles: profile::List,
        theme: Theme,
        error: Option<String>,
    },
}

pub async fn load() -> Message {
    let result = data::Saved::load().await.map_err(|e| e.to_string());
    Message::Loaded(result)
}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<'_, Message> {
        text("Loading").into()
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Loaded(result) => {
                let (saved, error) = match result {
                    Ok(saved) => (saved, None),
                    Err(err) => (data::Saved::default(), Some(err)),
                };
                let data::Saved {
                    profiles,
                    theme_name,
                    ..
                } = saved;
                Some(Event::Load {
                    profiles: profiles.into(),
                    theme: Theme::from_name(&theme_name).unwrap_or_default(),
                    error,
                })
            }
        }
    }
}
