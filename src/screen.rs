use iced::Subscription;

use crate::Element;
use crate::data::{self, Theme, profile};

pub mod loading;
pub mod mini_keyboard;
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
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Loading(loading::Message),
    Settings(settings::Message),
    Training(training::Message),
}

pub enum Event {
    ExitRequested,
    Save,
    SelectTheme(Theme),
    Toast(String),
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
        if let Screen::Settings(_) = self {
            *self = Screen::training(profiles);
        }
    }

    pub fn update(
        &mut self,
        profiles: &mut profile::List,
        message: Message,
        active: &str,
    ) -> Vec<Event> {
        let mut events = Vec::new();
        match self {
            Screen::Loading(state) => {
                if let Message::Loading(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            loading::Event::Load {
                                profiles: loaded,
                                theme,
                                error,
                            } => {
                                *profiles = loaded;
                                *self = Screen::training(profiles);
                                events.push(Event::SelectTheme(theme));
                                if let Some(err) = error {
                                    events.push(Event::Toast(err));
                                }
                            }
                        }
                    }
                }
            }
            Screen::Training(state) => {
                if let Message::Training(message) = message {
                    if let Some(event) = state.update(profiles, message) {
                        match event {
                            training::Event::Save => {
                                events.push(Event::Save);
                            }
                            training::Event::Settings => {
                                *self = Screen::settings();
                            }
                        }
                    }
                }
            }
            Screen::Settings(state) => {
                if let Message::Settings(message) = message {
                    if let Some(event) = state.update(profiles, message, active) {
                        match event {
                            settings::Event::Exit => {
                                *self = Screen::training(profiles);
                            }
                            settings::Event::Save => {
                                events.push(Event::Save);
                            }
                            settings::Event::SelectTheme(theme) => {
                                events.push(Event::SelectTheme(theme));
                            }
                        }
                    }
                }
            }
        }
        events
    }

    pub fn view(&self, profiles: &profile::List, theme: &data::Theme) -> Element<'_, Message> {
        match self {
            Screen::Loading(loading) => loading.view().map(Message::Loading),
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

impl Default for Screen {
    fn default() -> Self {
        Screen::new()
    }
}
