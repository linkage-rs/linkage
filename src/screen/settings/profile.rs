use crate::data::{self, Theme};
use crate::style;
use iced::button::{self, Button};
use iced::scrollable::{self, Scrollable};
use iced::{Column, Container, Element, Length, Row, Rule, Text};

#[derive(Debug)]
pub struct State {
    scroll_state: scrollable::State,
    profile_buttons: Vec<button::State>,
    new_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    ProfilePressed(usize),
    NewProfilePressed,
}

impl State {
    pub fn new() -> Self {
        Self {
            scroll_state: scrollable::State::new(),
            profile_buttons: Vec::new(),
            new_button: button::State::new(),
        }
    }

    pub fn update(&mut self, profiles: &mut data::profile::List, message: Message) {
        match message {
            Message::ProfilePressed(index) => {
                profiles.select(index);
            }
            Message::NewProfilePressed => {
                let profile = data::profile::Profile::new(profiles);
                profiles.insert_active(profile);
            }
        }
    }

    pub fn view(&mut self, profiles: &data::profile::List, theme: &Theme) -> Element<Message> {
        let State {
            scroll_state,
            profile_buttons,
            new_button,
        } = self;

        profile_buttons.resize(profiles.len(), button::State::new());

        let menu = Column::with_children(
            profiles
                .names()
                .zip(profile_buttons.iter_mut())
                .enumerate()
                .map(|(i, ((name, is_active), state))| {
                    let text = Container::new(Text::new(name).size(14))
                        .padding(6)
                        .center_x()
                        .center_y();
                    if is_active {
                        Container::new(text)
                            .style(style::container::menu_selected(theme))
                            .into()
                    } else {
                        Button::new(state, text)
                            .style(style::button::menu(theme, is_active))
                            .on_press(Message::ProfilePressed(i))
                            .padding(0)
                            .into()
                    }
                })
                .collect(),
        );
        let new_button = Button::new(new_button, Text::new("+ New Profile").size(14))
            .style(style::button::menu(theme, false))
            .on_press(Message::NewProfilePressed);
        let menu = Scrollable::new(scroll_state)
            .push(menu)
            .push(new_button)
            .height(Length::Fill);

        Row::new()
            .push(menu)
            .push(Rule::vertical(0).style(style::rule::divider(theme)))
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
