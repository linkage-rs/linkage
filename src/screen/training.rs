use iced::keyboard::key::Named;
use iced::keyboard::{self, Key};
use iced::widget::{Space, button, column, container, row, text};
use iced::{Alignment, Element, Length, Padding, Subscription, alignment};
use itertools::{EitherOrBoth, Itertools};

use crate::data::profile;
use crate::data::training::{CHARS_PER_LINE, Difficulty, MAX_ERRORS, MIN_CLEAN_PCT, TriplePoint};
use crate::font;

#[derive(Debug)]
pub struct State {
    modifiers: keyboard::Modifiers,
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

const CHAR_WIDTH: u32 = 10;
const ROW_CHARS: u32 = (CHARS_PER_LINE + MAX_ERRORS - 1) as u32;
const ROW_WIDTH: u32 = CHAR_WIDTH * ROW_CHARS;
const ROW_ERROR_WIDTH: u32 = (MAX_ERRORS - 1) as u32 * CHAR_WIDTH;
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

    pub fn view(&self, profiles: &profile::List) -> Element<'_, Message> {
        let active_line_children: Vec<Element<Message>> = profiles
            .session()
            .hits
            .iter()
            .map(|hit| {
                let mut t = text(hit.target().to_string())
                    .width(CHAR_WIDTH)
                    .font(iced::Font::from(font::Font::Thin));
                if hit.is_dirty() {
                    t = t.style(text::warning);
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
                                    .style(text::danger)
                                    .into()
                            }
                            EitherOrBoth::Right(t) => text(t.to_string()).width(CHAR_WIDTH).into(),
                        }
                    }),
            )
            .collect();

        let active_line = row(active_line_children);

        let target_indicator: Element<_> = if profiles.session().errors.is_empty() {
            row![
                Space::new().width(profiles.session().hits.len() as u32 * CHAR_WIDTH),
                text("\u{2015}")
                    .width(CHAR_WIDTH)
                    .height(LINE_SPACE)
                    .align_y(alignment::Vertical::Center)
                    .style(text::success),
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

        let error_pad = if STATS_WIDTH > ROW_ERROR_WIDTH {
            STATS_WIDTH - ROW_ERROR_WIDTH
        } else {
            0
        };

        let training =
            column![content_active, content_next].padding(Padding::ZERO.right(error_pad));
        let training = container(training)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

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
                        .style(move |theme: &iced::Theme| {
                            let palette = theme.palette();
                            let color = if accuracy_val < 0.5 {
                                let pct = accuracy_val / 0.5;
                                interpolate_color(palette.danger, palette.text, pct)
                            } else {
                                let pct = (accuracy_val - 0.5) / 0.5;
                                interpolate_color(palette.text, palette.success, pct)
                            };
                            text::Style { color: Some(color) }
                        })
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
                            .style(move |theme: &iced::Theme| {
                                let palette = theme.palette();
                                let color = if wpm_metric_value < 0.5 {
                                    let pct = wpm_metric_value / 0.5;
                                    interpolate_color(palette.danger, palette.text, pct)
                                } else {
                                    let pct = (wpm_metric_value - 0.5) / 0.5;
                                    interpolate_color(palette.text, palette.success, pct)
                                };
                                text::Style { color: Some(color) }
                            })
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

        let content = row![letter_stats, training]
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
            .style(button::text)
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
            _ => None,
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

fn interpolate_color(a: iced::Color, b: iced::Color, pct: f32) -> iced::Color {
    iced::Color {
        r: a.r + (b.r - a.r) * pct,
        g: a.g + (b.g - a.g) * pct,
        b: a.b + (b.b - a.b) * pct,
        a: a.a + (b.a - a.a) * pct,
    }
}
