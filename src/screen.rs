use crate::data::profile;
use crate::data::Theme;
use crate::Element;

use iced::Subscription;

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
        active: &'static str,
    ) -> Option<Event> {
        match self {
            Screen::Loading(state) => {
                if let Message::Loading(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            loading::Event::Load {
                                profiles: loaded,
                                theme,
                            } => {
                                *profiles = loaded;
                                *self = Screen::training(profiles);
                                return Some(Event::SelectTheme(theme));
                            }
                        }
                    }
                }
            }
            Screen::Training(state) => {
                if let Message::Training(message) = message {
                    if let Some((_command, event)) = state.update(profiles, message) {
                        match event {
                            training::Event::Save => {
                                return Some(Event::Save);
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
                                return Some(Event::Save);
                            }
                            settings::Event::SelectTheme(theme) => {
                                return Some(Event::SelectTheme(theme));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn view(&self, profiles: &profile::List) -> Element<Message> {
        match self {
            Screen::Loading(loading) => loading.view().map(Message::Loading),
            Screen::Settings(state) => state.view(profiles).map(Message::Settings),
            Screen::Training(state) => state.view(profiles).map(Message::Training),
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
