use crate::data::{self, Theme};
use crate::font;
use crate::style;

use iced::button::{self, Button};
use iced::{Column, Command, Element, Length, Row, Rule, Text};

mod profile;

#[derive(Debug)]
pub struct State {
    screen: Screen,
    back_button: button::State,
    menu_buttons: Vec<button::State>,
}

#[derive(Debug)]
pub enum Screen {
    Profile(profile::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    BackButtonPressed,
    ProfilesPressed,
    Profile(profile::Message),
}

pub enum Event {
    Exit,
}

impl State {
    pub fn new() -> Self {
        Self {
            screen: Screen::Profile(profile::State::new()),
            back_button: button::State::new(),
            menu_buttons: Vec::new(),
        }
    }

    pub fn update(
        &mut self,
        profiles: &mut data::profile::List,
        message: Message,
    ) -> Option<(Command<Message>, Event)> {
        match message {
            Message::BackButtonPressed => Some((Command::none(), Event::Exit)),
            _ => None,
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let State {
            screen,
            back_button,
            menu_buttons,
        } = self;

        let back_button = Button::new(back_button, Text::new("\u{2190} Back"))
            .on_press(Message::BackButtonPressed)
            .style(style::button::text(theme))
            .padding(10);

        let menu_items = vec![MenuItem {
            label: "Profiles",
            message: Message::ProfilesPressed,
            is_active: matches!(screen, Screen::Profile(..)),
        }];

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
                    let mut button =
                        Button::new(state, Text::new(label)).style(style::button::text(theme));
                    if !is_active {
                        button = button.on_press(message);
                    }
                    button.into()
                })
                .collect(),
        );

        let content = Row::new()
            .push(menu)
            .push(Rule::vertical(0).style(style::rule::divider(theme)))
            .push(screen.view(theme))
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
    fn view(&mut self, theme: &Theme) -> Element<Message> {
        match self {
            Screen::Profile(state) => state.view(theme).map(Message::Profile),
        }
    }
}
