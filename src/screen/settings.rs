use iced::widget::{Column, Row, button, column, container, rule, scrollable, text};
use iced::{Element, Length};

use crate::data::{self, Theme};

mod profile;
mod theme;

#[derive(Debug)]
pub struct State {
    screen: Screen,
}

#[derive(Debug)]
pub enum Screen {
    Profile(profile::State),
    Theme(theme::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    BackButtonPressed,
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
        active: &'static str,
    ) -> Option<Event> {
        let State { screen, .. } = self;
        match message {
            Message::BackButtonPressed => {
                return Some(Event::Exit);
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

    pub fn view(&self, profiles: &data::profile::List) -> Element<'_, Message> {
        let State { screen } = self;

        let back_button = button(text("\u{2190} Back").size(14))
            .on_press(Message::BackButtonPressed)
            .style(button::text)
            .padding(10);

        let menu_items = vec![
            MenuItem {
                label: "Profiles",
                message: Message::ProfilesPressed,
                is_active: matches!(screen, Screen::Profile(..)),
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
                        .center_x(Length::Fill)
                        .center_y(Length::Fill);
                    if is_active {
                        container(label_text)
                            .style(container::rounded_box)
                            .width(Length::Fill)
                            .into()
                    } else {
                        button(label_text)
                            .style(button::text)
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
            .push(screen.view(profiles))
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
    fn profile() -> Self {
        Screen::Profile(profile::State::new())
    }

    fn theme(active: &'static str) -> Self {
        Screen::Theme(theme::State::new(active))
    }

    fn view(&self, profiles: &data::profile::List) -> Element<'_, Message> {
        match self {
            Screen::Profile(state) => state.view(profiles).map(Message::Profile),
            Screen::Theme(state) => state.view().map(Message::Theme),
        }
    }
}
