use iced::Length;
use iced::widget::{Column, Row, button, column, container, rule, scrollable, text};

use crate::data::{self, Theme};
use crate::{Element, style};

pub mod keyboard;
mod profile;
mod theme;

#[derive(Debug)]
pub struct State {
    screen: Screen,
}

#[derive(Debug)]
pub enum Screen {
    Keyboard(keyboard::State),
    Profile(profile::State),
    Theme(theme::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    BackButtonPressed,
    Keyboard(keyboard::Message),
    KeyboardPressed,
    Profile(profile::Message),
    ProfilesPressed,
    Theme(theme::Message),
    ThemesPressed,
}

pub enum Event {
    Exit,
    Save,
    SelectTheme(Theme),
}

impl State {
    pub fn new() -> Self {
        Self {
            screen: Screen::profile(),
        }
    }

    pub fn update(
        &mut self,
        profiles: &mut data::profile::List,
        message: Message,
        active: &str,
    ) -> Option<Event> {
        let State { screen, .. } = self;
        match message {
            Message::BackButtonPressed => {
                return Some(Event::Exit);
            }
            Message::Keyboard(message) => {
                if let Screen::Keyboard(state) = screen {
                    state.update(profiles, message);
                    return Some(Event::Save);
                }
            }
            Message::KeyboardPressed => {
                *screen = Screen::keyboard(profiles);
            }
            Message::Profile(message) => {
                if let Screen::Profile(state) = screen {
                    if state.update(profiles, message) {
                        return Some(Event::Save);
                    }
                }
            }
            Message::ProfilesPressed => {
                *screen = Screen::profile();
            }
            Message::Theme(message) => {
                if let Screen::Theme(state) = screen {
                    if let Some(event) = state.update(message) {
                        match event {
                            theme::Event::SelectTheme(theme) => {
                                return Some(Event::SelectTheme(theme));
                            }
                        }
                    }
                }
            }
            Message::ThemesPressed => {
                *screen = Screen::theme(active);
            }
        }
        None
    }

    pub fn view(
        &self,
        profiles: &data::profile::List,
        theme: &data::Theme,
    ) -> Element<'_, Message> {
        let State { screen } = self;

        let back_button = button(text("\u{2190} Back").size(14))
            .on_press(Message::BackButtonPressed)
            .class(style::Button::Text)
            .padding(10);

        let menu_items = vec![
            MenuItem {
                label: "Profiles",
                message: Message::ProfilesPressed,
                is_active: matches!(screen, Screen::Profile(..)),
            },
            MenuItem {
                label: "Keyboard",
                message: Message::KeyboardPressed,
                is_active: matches!(screen, Screen::Keyboard(..)),
            },
            MenuItem {
                label: "Themes",
                message: Message::ThemesPressed,
                is_active: matches!(screen, Screen::Theme(_)),
            },
        ];

        let menu = Column::with_children(
            menu_items
                .into_iter()
                .map(|item| -> Element<Message> {
                    let MenuItem {
                        label,
                        message,
                        is_active,
                    } = item;
                    let label_text = container(text(label).size(14))
                        .padding(6)
                        .center_y(Length::Fill);
                    if is_active {
                        container(label_text)
                            .class(style::Container::MenuSelected)
                            .width(Length::Fill)
                            .into()
                    } else {
                        button(label_text)
                            .class(style::Button::Text)
                            .width(Length::Fill)
                            .on_press(message)
                            .padding(0)
                            .into()
                    }
                })
                .collect::<Vec<Element<Message>>>(),
        );
        let menu = container(scrollable(menu).height(Length::Fill)).width(125);

        let content = Row::new()
            .push(menu)
            .push(rule::vertical(1))
            .push(screen.view(profiles, theme))
            .height(Length::Fill)
            .width(Length::Fill);

        column![back_button, rule::horizontal(1), content,]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

struct MenuItem {
    pub label: &'static str,
    pub message: Message,
    pub is_active: bool,
}

impl Screen {
    fn keyboard(profiles: &data::profile::List) -> Self {
        Screen::Keyboard(keyboard::State::new(profiles))
    }

    fn profile() -> Self {
        Screen::Profile(profile::State::new())
    }

    fn theme(active: &str) -> Self {
        Screen::Theme(theme::State::new(active))
    }

    fn view(&self, profiles: &data::profile::List, theme: &data::Theme) -> Element<'_, Message> {
        match self {
            Screen::Keyboard(state) => state.view(profiles, theme).map(Message::Keyboard),
            Screen::Profile(state) => state.view(profiles).map(Message::Profile),
            Screen::Theme(state) => state.view().map(Message::Theme),
        }
    }
}
