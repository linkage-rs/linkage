use crate::data::User;
use iced::{Column, Command, Element, Subscription, Text};

#[derive(Debug)]
pub struct Training {
    user: User,
}

#[derive(Debug, Clone)]
pub enum Message {
    KeyboardEvent(iced::keyboard::Event),
    WindowFocused,
    WindowUnocused,
}

pub enum Event {
    ExitRequested,
}

impl Training {
    pub fn new(user: User) -> Self {
        Self { user }
    }

    pub fn update(&mut self, message: Message) -> Option<(Command<Message>, Event)> {
        match message {
            Message::KeyboardEvent(keyboard_event) => self.handle_keyboard(keyboard_event),
            _ => None,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new(format!("{} Training", self.user.name)))
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        use iced_native::event::{Event, Status};
        use iced_native::window::Event as WindowEvent;

        iced_native::subscription::events_with(|event, status| {
            if status == Status::Captured {
                return None;
            }
            match event {
                Event::Keyboard(keyboard_event) => Some(Message::KeyboardEvent(keyboard_event)),
                Event::Window(WindowEvent::Focused) => Some(Message::WindowFocused),
                Event::Window(WindowEvent::Unfocused) => Some(Message::WindowUnocused),
                _ => None,
            }
        })
    }

    pub fn handle_keyboard(
        &mut self,
        event: iced::keyboard::Event,
    ) -> Option<(Command<Message>, Event)> {
        use iced::keyboard::{self, KeyCode};

        match event {
            keyboard::Event::KeyPressed {
                key_code,
                modifiers: _modifiers,
            } => match key_code {
                KeyCode::Enter => None,
                KeyCode::Escape => None,
                #[cfg(target_os = "macos")]
                KeyCode::Q if _modifiers.is_command_pressed() => {
                    Some((Command::none(), Event::ExitRequested))
                }
                _ => None,
            },
            _ => None,
        }
    }
}
