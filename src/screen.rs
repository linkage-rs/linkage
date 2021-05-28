use iced::{Column, Command, Element, Subscription};

pub mod training;

#[derive(Debug)]
pub enum Screen {
    /// Startup screen when loading data from disk
    Loading,
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
    Training(training::Message),
}

pub enum Event {
    ExitRequested,
}

impl Screen {
    pub fn training() -> Self {
        Self::Training(training::Training::new())
    }

    pub fn update(&mut self, message: Message) -> Option<(Command<Message>, Event)> {
        match (self, message) {
            (Screen::Training(training), Message::Training(message)) => {
                match training.update(message) {
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
