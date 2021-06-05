use crate::data::{Freq, User};
use iced::{Column, Command, Element, Subscription};

pub mod loading;
pub mod training;

#[derive(Debug)]
pub enum Screen {
    /// Startup screen when loading data from disk
    Loading(loading::Loading),
    /// Tying practice
    Training(training::Training),
    /// Changing user settings
    Settings,
    /// Switching users
    UserSwitch,
    /// Shutting down
    Saving,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loading(loading::Message),
    Training(training::Message),
}

pub enum Event {
    ExitRequested,
    Training(User),
}

impl Screen {
    pub fn new() -> Self {
        Self::Loading(loading::Loading::new())
    }

    pub fn training(user: User, freq: &mut Freq) -> Self {
        Self::Training(training::Training::new(user, freq))
    }

    pub fn update(
        &mut self,
        message: Message,
        freq: &mut Freq,
    ) -> Option<(Command<Message>, Event)> {
        match (self, message) {
            (Screen::Loading(loading), Message::Loading(message)) => {
                match loading.update(message) {
                    Some(event) => match event {
                        loading::Event::Load(user) => {
                            Some((Command::none(), Event::Training(user)))
                        }
                    },
                    _ => None,
                }
            }
            (Screen::Training(training), Message::Training(message)) => {
                match training.update(message, freq) {
                    Some((command, event)) => match event {
                        training::Event::ExitRequested => {
                            Some((command.map(Message::Training), Event::ExitRequested))
                        }
                    },
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        match self {
            Screen::Loading(loading) => loading.view().map(Message::Loading),
            Screen::Training(training) => training.view().map(Message::Training),
            _ => Column::new().into(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self {
            Screen::Training(training) => training.subscription().map(Message::Training),
            _ => Subscription::none(),
        }
    }
}
