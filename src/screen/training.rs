use crate::data::profile;
use crate::data::training::{Difficulty, TriplePoint, CHARS_PER_LINE, MAX_ERRORS, MIN_CLEAN_PCT};
use crate::data::Theme;
use crate::font;
use crate::style;
use iced::alignment::{self, Alignment};
use iced::button::{self, Button};
use iced::keyboard::{self, KeyCode};
use iced::{Column, Command, Container, Element, Length, Row, Space, Subscription, Text};
use itertools::{EitherOrBoth, Itertools};

#[derive(Debug)]
pub struct State {
    modifiers: keyboard::Modifiers,
    settings_button: button::State,
    accuracy_metric: TriplePoint,
    wpm_metric: TriplePoint,
}

#[derive(Debug, Clone)]
pub enum Message {
    KeyboardEvent(iced::keyboard::Event),
    UserButtonPressed,
    WindowFocused,
    WindowUnocused,
}

pub enum Event {
    Save,
    Settings,
}

const CHAR_WIDTH: u16 = 10;
const ROW_CHARS: u16 = (CHARS_PER_LINE + MAX_ERRORS - 1) as u16;
const ROW_WIDTH: u16 = CHAR_WIDTH * ROW_CHARS;
const ROW_ERROR_WIDTH: u16 = (MAX_ERRORS - 1) as u16 * CHAR_WIDTH;
const LINE_SPACE: u16 = 10;
const STATS_WIDTH: u16 = 75;
pub const OVERALL_WIDTH: u16 = 2 * STATS_WIDTH + ROW_WIDTH;

impl State {
    pub fn new(difficulty: &Difficulty) -> Self {
        Self {
            modifiers: keyboard::Modifiers::default(),
            settings_button: button::State::new(),
            accuracy_metric: TriplePoint::new(0.5, MIN_CLEAN_PCT, 0.975).unwrap_or_default(),
            wpm_metric: TriplePoint::new(
                10.0,
                f64::from(difficulty.words_per_minute()) as f32,
                60.0,
            )
            .unwrap_or_default(),
        }
    }

    pub fn update(
        &mut self,
        profiles: &mut profile::List,
        message: Message,
    ) -> Option<(Command<Message>, Event)> {
        match message {
            Message::KeyboardEvent(keyboard_event) => {
                self.handle_keyboard(profiles, keyboard_event)
            }
            Message::UserButtonPressed => Some((Command::none(), Event::Settings)),
            _ => None,
        }
    }

    pub fn view(&mut self, profiles: &profile::List, theme: &Theme) -> Element<Message> {
        let active_line = Row::with_children(
            profiles
                .session()
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
                    profiles
                        .session()
                        .errors
                        .iter()
                        .zip_longest(
                            std::iter::once(&profiles.session().active_hit.target())
                                .chain(profiles.session().targets.iter()),
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

        let target_indicator: Element<_> = if profiles.session().errors.is_empty() {
            Row::with_children(vec![
                Space::with_width(Length::Units(
                    profiles.session().hits.len() as u16 * CHAR_WIDTH,
                ))
                .into(),
                Text::new("\u{2015}")
                    .width(Length::Units(CHAR_WIDTH))
                    .height(Length::Units(LINE_SPACE))
                    .vertical_alignment(alignment::Vertical::Center)
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
            profiles
                .session()
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

        let training = Column::with_children(vec![content_active.into(), content_next.into()])
            .padding([0, STATS_WIDTH.saturating_sub(ROW_ERROR_WIDTH), 0, 0]);
        let training = Container::new(training)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let letter_stats = Column::with_children(
            profiles
                .active()
                .state
                .clean_letters()
                .iter()
                .filter(|(ch, _)| *ch != ' ')
                .map(|(ch, val)| {
                    let stats = profiles.active().state.timings.get(ch);
                    let mut row = Row::new()
                        .push(Text::new(ch.to_string()).font(font::LIGHT).size(12))
                        .push(
                            Text::new("\u{25a0}")
                                .color(theme.metric(self.accuracy_metric.value(*val)))
                                .font(font::LIGHT)
                                .size(16),
                        )
                        .align_items(Alignment::Center)
                        .spacing(5);

                    if let Some(stats) = stats {
                        let wpm = f64::from(stats.wpm_harmonic_mean) as f32;
                        row = row.push(
                            Text::new("\u{25a0}")
                                .color(theme.metric(self.wpm_metric.value(wpm)))
                                .font(font::LIGHT)
                                .size(16),
                        )
                    }
                    row.into()
                })
                .collect(),
        )
        .width(Length::Units(STATS_WIDTH))
        .spacing(2)
        .padding(5);

        let content = Row::new()
            .push(letter_stats)
            .push(training)
            .width(Length::Fill)
            .height(Length::Fill);

        let settings_button_content = Column::new()
            .push(Text::new(profiles.active().name.clone()).size(14))
            .push(Text::new(profiles.active().layout.to_string()).size(14))
            .width(Length::Fill)
            .align_items(Alignment::End)
            .spacing(5);

        let settings_button = Button::new(&mut self.settings_button, settings_button_content)
            .on_press(Message::UserButtonPressed)
            .style(style::button::text(theme))
            .padding(10);

        let footer = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(settings_button);

        Column::with_children(vec![content.into(), footer.into()]).into()
    }

    pub fn handle_keyboard(
        &mut self,
        profiles: &mut profile::List,
        event: iced::keyboard::Event,
    ) -> Option<(Command<Message>, Event)> {
        match event {
            keyboard::Event::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers;
                None
            }

            keyboard::Event::KeyPressed { key_code, .. } => match key_code {
                KeyCode::Space => {
                    if let Some(line) = profiles.session_mut().apply_char(' ') {
                        if let Some(words) = profiles.active_mut().add_line(line) {
                            profiles.session_mut().update_words(words);
                        }
                        profiles.session_mut().fill_next_lines();
                        return Some((Command::none(), Event::Save));
                    }
                    None
                }
                KeyCode::Escape => None,
                KeyCode::Backspace => {
                    profiles.session_mut().backspace();
                    None
                }
                _ => None,
            },
            keyboard::Event::CharacterReceived(c)
                if c.is_alphanumeric() && !self.modifiers.command() =>
            {
                if let Some(line) = profiles.session_mut().apply_char(c) {
                    if let Some(words) = profiles.active_mut().add_line(line) {
                        profiles.session_mut().update_words(words);
                    }
                    profiles.session_mut().fill_next_lines();
                    return Some((Command::none(), Event::Save));
                }
                None
            }
            _ => None,
        }
    }
}

pub fn subscription() -> Subscription<Message> {
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
