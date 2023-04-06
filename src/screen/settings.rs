use crate::data::{self, Theme};
use crate::style;
use crate::Element;

use iced::widget::{container, Button, Column, Container, Row, Rule, Scrollable, Text};
use iced::Length;

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
        let State { ref mut screen, .. } = self;
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

    pub fn view(&self, profiles: &data::profile::List) -> Element<Message> {
        let State { screen } = self;

        let back_button = Button::new(Text::new("\u{2190} Back").size(14))
            .on_press(Message::BackButtonPressed)
            .style(style::Button::Text)
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
                .map(|item| {
                    let MenuItem {
                        label,
                        message,
                        is_active,
                    } = item;
                    let text = Container::new(Text::new(label).size(14))
                        .padding(6)
                        .center_x()
                        .center_y();
                    if is_active {
                        Container::new(text)
                            .style(style::Container::MenuSelected)
                            .width(Length::Fill)
                            .into()
                    } else {
                        Button::new(text)
                            .style(style::Button::Menu { selected: false })
                            .width(Length::Fill)
                            .on_press(message)
                            .padding(0)
                            .into()
                    }
                })
                .collect(),
        );
        let menu = container(Scrollable::new(menu).height(Length::Fill)).width(125);

        let content = Row::new()
            .push(menu)
            .push(Rule::vertical(1).style(style::Rule::Divider))
            .push(screen.view(profiles))
            .height(Length::Fill)
            .width(Length::Fill);

        Column::new()
            .push(back_button)
            .push(Rule::horizontal(1).style(style::Rule::Divider))
            .push(content)
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

    fn view(&self, profiles: &data::profile::List) -> Element<Message> {
        match self {
            Screen::Profile(state) => state.view(profiles).map(Message::Profile),
            Screen::Theme(state) => state.view().map(Message::Theme),
        }
    }
}
