use crate::data::profile;
use crate::data::Theme;
use iced::{Element, Subscription};

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
    Save,
    SelectTheme(Theme),
}

impl Screen {
    pub fn new() -> Self {
        Self::Loading(loading::State::new())
    }

    pub fn settings() -> Self {
        Self::Settings(settings::State::new())
    }

    pub fn training(profiles: &profile::List) -> Self {
        Self::Training(training::State::new(&profiles.active().difficulty))
    }

    pub fn go_back(&mut self, profiles: &profile::List) {
        match self {
            Screen::Settings(..) => {
                *self = Screen::training(profiles);
            }
            _ => {}
        }
    }

    pub fn update(&mut self, profiles: &mut profile::List, message: Message) -> Option<Event> {
        match self {
            Screen::Loading(state) => match message {
                Message::Loading(message) => match state.update(message) {
                    Some(event) => match event {
                        loading::Event::Load {
                            profiles: loaded,
                            theme,
                        } => {
                            *profiles = loaded;
                            *self = Screen::training(&profiles);
                            return Some(Event::SelectTheme(theme));
                        }
                    },
                    None => {}
                },
                _ => {}
            },
            Screen::Training(state) => match message {
                Message::Training(message) => match state.update(profiles, message) {
                    Some((_command, event)) => match event {
                        training::Event::Save => {
                            return Some(Event::Save);
                        }
                        training::Event::Settings => {
                            *self = Screen::settings();
                        }
                    },
                    None => {}
                },
                _ => {}
            },
            Screen::Settings(state) => match message {
                Message::Settings(message) => match state.update(profiles, message) {
                    Some(event) => match event {
                        settings::Event::Exit => {
                            *self = Screen::training(&profiles);
                        }
                        settings::Event::Save => {
                            return Some(Event::Save);
                        }
                        settings::Event::SelectTheme(theme) => {
                            return Some(Event::SelectTheme(theme));
                        }
                    },
                    None => {}
                },
                _ => {}
            },
        }
        None
    }

    pub fn view(&mut self, profiles: &profile::List, theme: &Theme) -> Element<Message> {
        match self {
            Screen::Loading(loading) => loading.view(theme).map(Message::Loading),
            Screen::Settings(state) => state.view(profiles, theme).map(Message::Settings),
            Screen::Training(state) => state.view(profiles, theme).map(Message::Training),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self {
            Screen::Training { .. } => training::subscription().map(Message::Training),
            _ => Subscription::none(),
        }
    }
}
