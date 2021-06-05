use crate::data::training::Session;
use crate::data::{Freq, User};
use iced::keyboard::{self, KeyCode};
use iced::{Column, Command, Element, Row, Subscription, Text};

#[derive(Debug)]
pub struct Training {
    user: User,
    session: Session,
    modifiers: keyboard::Modifiers,
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
    pub fn new(user: User, freq: &mut Freq) -> Self {
        let session = user.profile().start_session(freq);
        Self {
            user,
            session,
            modifiers: keyboard::Modifiers::default(),
        }
    }

    pub fn update(
        &mut self,
        message: Message,
        freq: &mut Freq,
    ) -> Option<(Command<Message>, Event)> {
        match message {
            Message::KeyboardEvent(keyboard_event) => self.handle_keyboard(keyboard_event, freq),
            _ => None,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        // TODO: fold the iterator to join consecutive hits/misses into strings
        // TODO: color by hit / miss
        let hits_part = self
            .session
            .hits
            .iter()
            .fold(String::new(), |s, h| format!("{}{}", s, h.target()));
        let targets_part = self
            .session
            .targets
            .iter()
            .fold(String::new(), |s, h| format!("{}{}", s, h));
        // TODO errors display then the rest of the targets

        let active_line = Row::with_children(vec![
            Text::new(hits_part).into(),
            Text::new(self.session.active_hit.target().to_string()).into(),
            Text::new(targets_part).into(),
        ]);
        let mut content = Column::new()
            .spacing(10)
            .push(Text::new(format!("{} Training", self.user.name)))
            .push(active_line);

        for line in &self.session.next_lines {
            content = content.push(Text::new(line))
        }

        content.into()
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
        freq: &mut Freq,
    ) -> Option<(Command<Message>, Event)> {
        match event {
            keyboard::Event::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers;
                None
            }

            keyboard::Event::KeyPressed {
                key_code,
                modifiers,
            } => match key_code {
                KeyCode::Space => {
                    if let Some(mut events) = self.session.apply_char(' ', freq) {
                        self.user.profile_mut().add_events(&mut events);
                    }
                    None
                }
                KeyCode::Escape => None,
                KeyCode::Backspace => {
                    self.session.backspace();
                    None
                }
                #[cfg(target_os = "macos")]
                KeyCode::Q if modifiers.is_command_pressed() => {
                    Some((Command::none(), Event::ExitRequested))
                }
                _ => None,
            },
            keyboard::Event::CharacterReceived(c)
                if c.is_alphanumeric() && !self.modifiers.is_command_pressed() =>
            {
                println!("CharacterReceived('{}')", c);
                if let Some(mut events) = self.session.apply_char(c, freq) {
                    self.user.profile_mut().add_events(&mut events);
                }
                None
            }
            _ => None,
        }
    }
}
