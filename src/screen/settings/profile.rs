use crate::data::{self, Theme};
use crate::style;
use iced::button::{self, Button};
use iced::scrollable::{self, Scrollable};
use iced::text_input::{self, TextInput};
use iced::{Column, Container, Element, Length, Row, Rule, Text};

#[derive(Debug)]
pub struct State {
    content_scroll: scrollable::State,
    menu_scroll: scrollable::State,
    name_input: text_input::State,
    name_parsed: Option<data::profile::Name>,
    name_value: String,
    new_button: button::State,
    profile_buttons: Vec<button::State>,
    rename_accept: button::State,
    rename_button: button::State,
    rename_cancel: button::State,
    renaming: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameInput(String),
    NewProfilePressed,
    ProfilePressed(usize),
    RenameAccept,
    RenameCancel,
    RenamePressed,
}

impl State {
    pub fn new() -> Self {
        Self {
            content_scroll: scrollable::State::new(),
            menu_scroll: scrollable::State::new(),
            name_input: text_input::State::new(),
            name_parsed: None,
            name_value: String::new(),
            new_button: button::State::new(),
            profile_buttons: Vec::new(),
            rename_accept: button::State::new(),
            rename_button: button::State::new(),
            rename_cancel: button::State::new(),
            renaming: false,
        }
    }

    pub fn update(&mut self, profiles: &mut data::profile::List, message: Message) {
        match message {
            Message::NameInput(new_name) => {
                self.name_parsed = None;
                if let Some(name) = data::profile::Name::parse(&new_name) {
                    if !profiles.contains_name(&name) {
                        self.name_parsed = Some(name);
                    }
                }
                self.name_value = new_name;
            }
            Message::NewProfilePressed => {
                let profile = data::profile::Profile::new(profiles);
                profiles.insert_active(profile);
            }
            Message::ProfilePressed(index) => {
                profiles.select(index);
            }
            Message::RenameAccept => {
                if let Some(name) = &self.name_parsed {
                    profiles.active_mut().name = name.clone();
                }
                self.renaming = false;
            }
            Message::RenameCancel => {
                self.renaming = false;
            }
            Message::RenamePressed => {
                self.name_parsed = None;
                self.name_value = profiles.active().name.to_string();
                self.renaming = true;
            }
        }
    }

    pub fn view(&mut self, profiles: &data::profile::List, theme: &Theme) -> Element<Message> {
        let State {
            content_scroll,
            menu_scroll,
            name_input,
            name_parsed,
            name_value,
            new_button,
            profile_buttons,
            rename_accept,
            rename_button,
            rename_cancel,
            renaming,
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
                            .width(Length::Fill)
                            .into()
                    } else {
                        Button::new(state, text)
                            .style(style::button::menu(theme, is_active))
                            .on_press(Message::ProfilePressed(i))
                            .width(Length::Fill)
                            .padding(0)
                            .into()
                    }
                })
                .collect(),
        )
        .width(Length::Fill);

        let new_button = Button::new(new_button, Text::new("+ New Profile").size(14))
            .style(style::button::menu(theme, false))
            .on_press(Message::NewProfilePressed)
            .width(Length::Fill);

        let menu = Scrollable::new(menu_scroll)
            .push(menu)
            .push(new_button)
            .height(Length::Fill)
            .width(Length::Units(175));

        let mut content = Scrollable::new(content_scroll).height(Length::Fill);

        if *renaming {
            let name_input =
                TextInput::new(name_input, "Profile Name", name_value, Message::NameInput)
                    .style(style::text_input::themed(theme))
                    .width(Length::Fill);
            let mut rename_accept = Button::new(rename_accept, Text::new("\u{2714}"));
            if name_parsed.is_some() {
                rename_accept = rename_accept.on_press(Message::RenameAccept);
            }
            let rename_cancel =
                Button::new(rename_cancel, Text::new("\u{2716}")).on_press(Message::RenameCancel);
            let name_row = Row::new()
                .push(name_input)
                .push(rename_accept)
                .push(rename_cancel);
            content = content.push(name_row);
        } else {
            let rename_button =
                Button::new(rename_button, Text::new(profiles.active().name.to_string()))
                    .style(style::button::text(theme))
                    .on_press(Message::RenamePressed);
            content = content.push(rename_button)
        }

        Row::new()
            .push(menu)
            .push(Rule::vertical(0).style(style::rule::divider(theme)))
            .push(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
