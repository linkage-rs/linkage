use std::collections::HashSet;

use iced::keyboard::key::Named;
use iced::keyboard::{self, Key};
use iced::widget::{Space, button, column, container, row, text};
use iced::{Alignment, Length, Subscription, alignment};
use itertools::{EitherOrBoth, Itertools};

use crate::data::profile;
use crate::data::training::{CHARS_PER_LINE, Difficulty, MIN_CLEAN_PCT, TriplePoint};
use crate::screen::mini_keyboard::{MiniKeyboard, MiniKeyboardData};
use crate::{Element, font, style};

#[derive(Debug)]
pub struct State {
    modifiers: keyboard::Modifiers,
    accuracy_metric: TriplePoint,
    wpm_metric: TriplePoint,
    pressed_keys: HashSet<char>,
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

const CHAR_WIDTH: u32 = 10;
const ROW_WIDTH: u32 = CHAR_WIDTH * CHARS_PER_LINE as u32;
const LINE_SPACE: u32 = 10;
const STATS_WIDTH: u32 = 75;
pub const OVERALL_WIDTH: u32 = 2 * STATS_WIDTH + ROW_WIDTH;

impl State {
    pub fn new(difficulty: &Difficulty) -> Self {
        Self {
            modifiers: keyboard::Modifiers::default(),
            accuracy_metric: TriplePoint::new(0.5, MIN_CLEAN_PCT, 0.975).unwrap_or_default(),
            wpm_metric: TriplePoint::new(10.0, f32::from(difficulty.words_per_minute()), 60.0)
                .unwrap_or_default(),
            pressed_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self, profiles: &mut profile::List, message: Message) -> Option<Event> {
        match message {
            Message::KeyboardEvent(keyboard_event) => {
                self.handle_keyboard(profiles, keyboard_event)
            }
            Message::UserButtonPressed => Some(Event::Settings),
            _ => None,
        }
    }

    pub fn view(
        &self,
        profiles: &profile::List,
        theme: &crate::data::Theme,
    ) -> Element<'_, Message> {
        let active_line_children: Vec<Element<Message>> = profiles
            .session()
            .hits
            .iter()
            .map(|hit| {
                let mut t = text(hit.target().to_string())
                    .width(CHAR_WIDTH)
                    .font(iced::Font::from(font::Font::Thin));
                if hit.is_dirty() {
                    t = t.class(style::Text::Miss);
                }
                t.into()
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
                    .map(|result| -> Element<Message> {
                        match result {
                            EitherOrBoth::Left(e) | EitherOrBoth::Both(e, _) => {
                                let c = if *e == ' ' { '\u{2591}' } else { *e };
                                text(c.to_string())
                                    .width(CHAR_WIDTH)
                                    .font(iced::Font::from(font::Font::Medium))
                                    .class(style::Text::Error)
                                    .into()
                            }
                            EitherOrBoth::Right(t) => text(t.to_string()).width(CHAR_WIDTH).into(),
                        }
                    }),
            )
            .take(CHARS_PER_LINE)
            .collect();

        let active_line = row(active_line_children);

        let target_indicator: Element<_> = if profiles.session().errors.is_empty() {
            row![
                Space::new().width(profiles.session().hits.len() as u32 * CHAR_WIDTH),
                text("\u{2015}")
                    .width(CHAR_WIDTH)
                    .height(LINE_SPACE)
                    .align_y(alignment::Vertical::Center)
                    .class(style::Text::Target),
            ]
            .into()
        } else {
            Space::new().height(LINE_SPACE).into()
        };

        let content_active = column![active_line, target_indicator].width(ROW_WIDTH);

        let next_lines_children: Vec<Element<Message>> = profiles
            .session()
            .next_lines
            .iter()
            .map(|line| {
                row(line
                    .chars()
                    .map(|c| -> Element<Message> { text(c.to_string()).width(CHAR_WIDTH).into() })
                    .collect::<Vec<_>>())
                .into()
            })
            .collect();

        let content_next = column(next_lines_children)
            .spacing(LINE_SPACE)
            .width(ROW_WIDTH);

        let training = column![content_active, content_next];
        let mut center_column = column![
            container(training)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        ]
        .align_x(Alignment::Center);
        if profiles.active().mini_keyboard.show {
            let mini_keyboard = MiniKeyboard::view(MiniKeyboardData::new(
                &profiles.active().layout,
                &profiles.active().mini_keyboard,
                &profiles.active().state,
                &self.accuracy_metric,
                &self.wpm_metric,
                theme,
                &self.pressed_keys,
            ));
            let keyboard_container = container(mini_keyboard)
                .center_x(Length::Fill)
                .center_y(Length::Fixed(100.0));
            center_column = center_column.push(keyboard_container);
        };

        let letter_stats_children: Vec<Element<Message>> = profiles
            .active()
            .state
            .clean_letters()
            .iter()
            .filter(|(ch, _)| *ch != ' ')
            .map(|(ch, val)| {
                let stats = profiles.active().state.timings.get(ch);
                let accuracy_val = self.accuracy_metric.value(*val);
                let mut r = row![
                    text(ch.to_string())
                        .font(iced::Font::from(font::Font::Light))
                        .size(12),
                    text("\u{25a0}")
                        .class(style::Text::Metric(accuracy_val))
                        .font(iced::Font::from(font::Font::Light))
                        .size(16),
                ]
                .align_y(Alignment::Center)
                .spacing(5);

                if let Some(stats) = stats {
                    let wpm = f64::from(stats.wpm_harmonic_mean) as f32;
                    let wpm_metric_value = self.wpm_metric.value(wpm);
                    r = r.push(
                        text("\u{25a0}")
                            .class(style::Text::Metric(wpm_metric_value))
                            .font(iced::Font::from(font::Font::Light))
                            .size(16),
                    )
                }
                r.into()
            })
            .collect();

        let letter_stats = column(letter_stats_children)
            .width(STATS_WIDTH)
            .spacing(2)
            .padding(5);

        let content = row![letter_stats, center_column, Space::new().width(STATS_WIDTH)]
            .width(Length::Fill)
            .height(Length::Fill);

        let settings_button_content = column![
            text(profiles.active().name.to_string()).size(14),
            text(profiles.active().layout.to_string()).size(14),
        ]
        .width(Length::Fill)
        .align_x(Alignment::End)
        .spacing(5);

        let settings_button = button(settings_button_content)
            .on_press(Message::UserButtonPressed)
            .class(style::Button::Text)
            .padding(10);

        let footer = row![Space::new().width(Length::Fill), settings_button];

        column![content, footer].into()
    }

    pub fn handle_keyboard(
        &mut self,
        profiles: &mut profile::List,
        event: iced::keyboard::Event,
    ) -> Option<Event> {
        match event {
            keyboard::Event::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers;
                None
            }

            keyboard::Event::KeyPressed {
                key,
                text: key_text,
                modifiers,
                ..
            } => {
                // Track pressed keys for mini keyboard highlight
                match key.as_ref() {
                    Key::Named(Named::Space) => {
                        self.pressed_keys.insert(' ');
                    }
                    _ => {
                        if let Some(txt) = &key_text {
                            for c in txt.chars() {
                                self.pressed_keys.insert(c.to_ascii_lowercase());
                            }
                        }
                    }
                }

                match key.as_ref() {
                    Key::Named(Named::Space) => {
                        if let Some(line) = profiles.session_mut().apply_char(' ') {
                            if let Some(words) = profiles.active_mut().add_line(line) {
                                profiles.session_mut().update_words(words);
                            }
                            let profile = profiles.active_mut();
                            profile.session.fill_next_lines(&profile.state);
                            return Some(Event::Save);
                        }
                        None
                    }
                    Key::Named(Named::Escape) => None,
                    Key::Named(Named::Backspace) => {
                        profiles.session_mut().backspace();
                        None
                    }
                    _ => {
                        // Handle character input via the `text` field
                        if modifiers.command() {
                            return None;
                        }
                        if let Some(txt) = key_text {
                            for c in txt.chars() {
                                if c.is_alphanumeric() {
                                    if let Some(line) = profiles.session_mut().apply_char(c) {
                                        if let Some(words) = profiles.active_mut().add_line(line) {
                                            profiles.session_mut().update_words(words);
                                        }
                                        let profile = profiles.active_mut();
                                        profile.session.fill_next_lines(&profile.state);
                                        return Some(Event::Save);
                                    }
                                }
                            }
                        }
                        None
                    }
                }
            }

            keyboard::Event::KeyReleased { key, .. } => {
                // Untrack released keys for mini keyboard highlight
                match key.as_ref() {
                    Key::Named(Named::Space) => {
                        self.pressed_keys.remove(&' ');
                    }
                    Key::Character(txt) => {
                        for c in txt.chars() {
                            self.pressed_keys.remove(&c.to_ascii_lowercase());
                        }
                    }
                    _ => {}
                }
                None
            }
        }
    }
}

pub fn subscription() -> Subscription<Message> {
    use iced::event::{Event, Status};
    use iced::window::Event as WindowEvent;

    iced::event::listen_with(|event, status, _id| {
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
