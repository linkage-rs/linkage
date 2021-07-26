use crate::data::{self, Theme};
use crate::style;

use iced::button::{self, Button};
use iced::scrollable::{self, Scrollable};
use iced::{Column, Command, Container, Element, Length, Row, Rule, Text};

mod profile;
mod theme;

#[derive(Debug)]
pub struct State {
    screen: Screen,
    back_button: button::State,
    menu_buttons: Vec<button::State>,
    menu_scroll: scrollable::State,
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
            back_button: button::State::new(),
            menu_buttons: Vec::new(),
            menu_scroll: scrollable::State::new(),
        }
    }

    pub fn update(
        &mut self,
        profiles: &mut data::profile::List,
        message: Message,
    ) -> Option<(Command<Message>, Event)> {
        let State { ref mut screen, .. } = self;
        match message {
            Message::BackButtonPressed => {
                return Some((Command::none(), Event::Exit));
            }
            Message::Profile(message) => {
                if let Screen::Profile(state) = screen {
                    if state.update(profiles, message) {
                        return Some((Command::none(), Event::Save));
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
                                return Some((Command::none(), Event::SelectTheme(theme)));
                            }
                        }
                    }
                }
            }
            Message::ThemesPressed => {
                *screen = Screen::theme();
            }
        }
        None
    }

    pub fn view(&mut self, profiles: &data::profile::List, theme: &Theme) -> Element<Message> {
        let State {
            screen,
            back_button,
            menu_buttons,
            menu_scroll,
        } = self;

        let back_button = Button::new(back_button, Text::new("\u{2190} Back").size(14))
            .on_press(Message::BackButtonPressed)
            .style(style::button::text(theme))
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

        menu_buttons.resize(menu_items.len(), button::State::new());

        let menu = Column::with_children(
            menu_items
                .into_iter()
                .zip(menu_buttons)
                .map(|(item, state)| {
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
                            .style(style::container::menu_selected(theme))
                            .width(Length::Fill)
                            .into()
                    } else {
                        Button::new(state, text)
                            .style(style::button::menu(theme, is_active))
                            .width(Length::Fill)
                            .on_press(message)
                            .padding(0)
                            .into()
                    }
                })
                .collect(),
        );
        let menu = Scrollable::new(menu_scroll)
            .push(menu)
            .height(Length::Fill)
            .width(Length::Units(125));

        let content = Row::new()
            .push(menu)
            .push(Rule::vertical(0).style(style::rule::divider(theme)))
            .push(screen.view(profiles, theme))
            .height(Length::Fill)
            .width(Length::Fill);

        Column::new()
            .push(back_button)
            .push(Rule::horizontal(0).style(style::rule::divider(theme)))
            .push(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
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

    fn theme() -> Self {
        Screen::Theme(theme::State::new())
    }

    fn view(&mut self, profiles: &data::profile::List, theme: &Theme) -> Element<Message> {
        match self {
            Screen::Profile(state) => state.view(profiles, theme).map(Message::Profile),
            Screen::Theme(state) => state.view(theme).map(Message::Theme),
        }
    }
}
