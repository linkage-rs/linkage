use crate::data::training::{Session, CHARS_PER_LINE, MAX_ERRORS};
use crate::data::user;
use crate::data::Theme;
use crate::font;
use iced::keyboard::{self, KeyCode};
use iced::{Column, Command, Element, Length, Row, Space, Subscription, Text, VerticalAlignment};
use itertools::{EitherOrBoth, Itertools};

#[derive(Debug)]
pub struct Training {
    users: user::List,
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

const CHAR_WIDTH: u16 = 10;
const ROW_CHARS: u16 = (CHARS_PER_LINE + MAX_ERRORS - 1) as u16;
const ROW_WIDTH: u16 = CHAR_WIDTH * ROW_CHARS;
const ROW_ERROR_WIDTH: u16 = (MAX_ERRORS - 1) as u16 * CHAR_WIDTH;
const LINE_SPACE: u16 = 10;

impl Training {
    pub fn new(users: user::List) -> Self {
        Self {
            users,
            modifiers: keyboard::Modifiers::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<(Command<Message>, Event)> {
        match message {
            Message::KeyboardEvent(keyboard_event) => self.handle_keyboard(keyboard_event),
            _ => None,
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let active_line = Row::with_children(
            self.session()
                .hits
                .iter()
                .map(|hit| {
                    Text::new(hit.target().to_string())
                        .width(Length::Units(CHAR_WIDTH))
                        .font(font::THIN)
                        .color(if hit.is_dirty() {
                            theme.miss
                        } else {
                            theme.text
                        })
                })
                .chain(
                    self.session()
                        .errors
                        .iter()
                        .zip_longest(
                            std::iter::once(&self.session().active_hit.target())
                                .chain(self.session().targets.iter()),
                        )
                        .map(|result| match result {
                            EitherOrBoth::Left(e) | EitherOrBoth::Both(e, _) => {
                                let c = if *e == ' ' { '\u{2591}' } else { *e };
                                Text::new(c.to_string())
                                    .width(Length::Units(CHAR_WIDTH))
                                    .font(font::MEDIUM)
                                    .color(theme.error)
                            }
                            EitherOrBoth::Right(t) => {
                                Text::new(t.to_string()).width(Length::Units(CHAR_WIDTH))
                            }
                        }),
                )
                .map(|text| text.into())
                .collect(),
        );

        let target_indicator: Element<_> = if self.session().errors.is_empty() {
            Row::with_children(vec![
                Space::with_width(Length::Units(self.session().hits.len() as u16 * CHAR_WIDTH))
                    .into(),
                Text::new("\u{2015}")
                    .width(Length::Units(CHAR_WIDTH))
                    .height(Length::Units(LINE_SPACE))
                    .vertical_alignment(VerticalAlignment::Center)
                    .color(theme.target)
                    .into(),
            ])
            .into()
        } else {
            Space::with_height(Length::Units(LINE_SPACE)).into()
        };

        let content_active = Column::new()
            .width(Length::Units(ROW_WIDTH))
            .push(active_line)
            .push(target_indicator);

        let content_next = Column::with_children(
            self.session()
                .next_lines
                .iter()
                .map(|line| {
                    Row::with_children(
                        line.chars()
                            .map(|c| {
                                Text::new(c.to_string())
                                    .width(Length::Units(CHAR_WIDTH))
                                    .into()
                            })
                            .collect(),
                    )
                    .into()
                })
                .collect(),
        )
        .spacing(LINE_SPACE)
        .width(Length::Units(ROW_WIDTH));

        Column::with_children(vec![content_active.into(), content_next.into()])
            .padding([0, 0, 0, ROW_ERROR_WIDTH])
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
                    if let Some(line) = self.session_mut().apply_char(' ') {
                        if let Some(words) = self.users.profile_mut().add_line(line) {
                            self.session_mut().update_words(words)
                        }
                    }
                    None
                }
                KeyCode::Escape => None,
                KeyCode::Backspace => {
                    self.session_mut().backspace();
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
                if let Some(line) = self.session_mut().apply_char(c) {
                    if let Some(words) = self.users.profile_mut().add_line(line) {
                        self.session_mut().update_words(words)
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn session(&self) -> &Session {
        &self.users.active().profiles.active().session
    }

    fn session_mut(&mut self) -> &mut Session {
        &mut self.users.active_mut().profiles.active_mut().session
    }
}
