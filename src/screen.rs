use crate::data::user;
use crate::data::Theme;
use iced::{Command, Element, Subscription};

pub mod loading;
mod settings;
pub mod training;

#[derive(Debug)]
pub enum Screen {
    /// Startup screen when loading data from disk
    Loading(loading::State),
    /// Tying practice
    Training(training::State),
    /// Changing user settings
    Settings(settings::State),
    // /// Shutting down
    // Saving,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loading(loading::Message),
    Settings(settings::Message),
    Training(training::Message),
}

pub enum Event {
    ExitRequested,
}

impl Screen {
    pub fn new() -> Self {
        Self::Loading(loading::State::new())
    }

    pub fn settings() -> Self {
        Self::Settings(settings::State::new())
    }

    pub fn training() -> Self {
        Self::Training(training::State::new())
    }

    pub fn update(
        &mut self,
        users: &mut user::List,
        message: Message,
    ) -> Option<(Command<Message>, Event)> {
        match self {
            Screen::Loading(state) => match message {
                Message::Loading(message) => match state.update(message) {
                    Some(event) => match event {
                        loading::Event::Load(loaded_users) => {
                            *self = Screen::training();
                            *users = loaded_users;
                            None
                        }
                    },
                    None => None,
                },
                _ => None,
            },
            Screen::Training(state) => match message {
                Message::Training(message) => match state.update(users, message) {
                    Some((command, event)) => match event {
                        training::Event::ExitRequested => {
                            Some((command.map(Message::Training), Event::ExitRequested))
                        }
                        training::Event::Settings => {
                            *self = Screen::settings();
                            None
                        }
                    },
                    None => None,
                },
                _ => None,
            },
            Screen::Settings(state) => match message {
                Message::Settings(message) => match state.update(users, message) {
                    Some((_command, event)) => match event {
                        settings::Event::Exit => {
                            *self = Screen::training();
                            None
                        }
                    },
                    None => None,
                },
                _ => None,
            },
        }
    }

    pub fn view(&mut self, users: &user::List, theme: &Theme) -> Element<Message> {
        match self {
            Screen::Loading(loading) => loading.view(theme).map(Message::Loading),
            Screen::Settings(state) => state.view(theme).map(Message::Settings),
            Screen::Training(state) => state.view(users, theme).map(Message::Training),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self {
            Screen::Training { .. } => training::subscription().map(Message::Training),
            _ => Subscription::none(),
        }
    }
}
