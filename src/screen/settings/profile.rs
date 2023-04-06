use crate::data::keyboard::{self, Layout};
use crate::data::profile;
use crate::data::training::Difficulty;
use crate::font;
use crate::style;
use crate::Element;

use iced::widget::{
    container, Button, Column, Container, PickList, Row, Rule, Scrollable, Text, TextInput,
};
use iced::Length;

#[derive(Debug)]
pub struct State {
    menu: Menu,
    screen: Screen,
}

#[derive(Debug)]
pub struct Menu {}

#[derive(Debug)]
pub enum Screen {
    Create {
        difficulty: Option<Difficulty>,
        layout: Option<Layout>,
        name_parsed: Option<profile::Name>,
        name_value: String,
    },
    Rename {
        name_parsed: Option<profile::Name>,
        name_value: String,
    },
    View,
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateAccept,
    CreateCancel,
    DifficultyChanged(Difficulty),
    LayoutChanged(Layout),
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
            menu: Menu::new(),
            screen: Screen::viewing(),
        }
    }

    pub fn update(&mut self, profiles: &mut profile::List, message: Message) -> bool {
        match message {
            Message::CreateAccept => {
                if let Screen::Create {
                    difficulty: Some(difficulty),
                    layout: Some(layout),
                    name_parsed: Some(name_parsed),
                    ..
                } = &self.screen
                {
                    let profile = profile::Profile::new(name_parsed.clone(), *layout, *difficulty);
                    profiles.insert_active(profile);

                    self.screen = Screen::viewing();

                    return true;
                }
            }
            Message::CreateCancel => {
                self.screen = Screen::viewing();
            }
            Message::DifficultyChanged(new_difficulty) => match self.screen {
                Screen::Create {
                    ref mut difficulty, ..
                } => {
                    *difficulty = Some(new_difficulty);
                }
                Screen::View { .. } => {
                    profiles.active_mut().difficulty = new_difficulty;

                    return true;
                }
                _ => {}
            },
            Message::LayoutChanged(new_layout) => {
                if let Screen::Create { ref mut layout, .. } = &mut self.screen {
                    *layout = Some(new_layout);
                }
            }
            Message::NameInput(new_name) => match &mut self.screen {
                Screen::Create {
                    ref mut name_parsed,
                    ref mut name_value,
                    ..
                }
                | Screen::Rename {
                    ref mut name_parsed,
                    ref mut name_value,
                    ..
                } => {
                    *name_parsed = None;
                    if let Some(name) = profile::Name::parse(&new_name) {
                        if !profiles.contains_name(&name) {
                            *name_parsed = Some(name);
                        }
                    }
                    *name_value = new_name;
                }
                _ => {}
            },
            Message::NewProfilePressed => {
                self.screen = Screen::creating();
            }
            Message::ProfilePressed(index) => {
                profiles.select(index);
                self.screen = Screen::viewing();
                return true;
            }
            Message::RenameAccept => {
                if let Screen::Rename {
                    name_parsed: Some(name_parsed),
                    ..
                } = &self.screen
                {
                    profiles.active_mut().name = name_parsed.clone();
                    self.screen = Screen::viewing();

                    return true;
                }
            }
            Message::RenameCancel => {
                self.screen = Screen::viewing();
            }
            Message::RenamePressed => {
                self.screen = Screen::renaming(profiles.active().name.to_string());
            }
        }
        false
    }

    pub fn view(&self, profiles: &profile::List) -> Element<Message> {
        let State { menu, screen } = self;

        let menu = menu.view(profiles);

        let content = Scrollable::new(
            container(screen.view(profiles))
                .width(Length::Fill)
                .padding(10),
        )
        .height(Length::Fill);

        Row::new()
            .push(menu)
            .push(Rule::vertical(0).style(style::Rule::Divider))
            .push(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl Menu {
    fn new() -> Self {
        Self {}
    }

    fn view(&self, profiles: &profile::List) -> Element<Message> {
        let menu = Column::with_children(
            profiles
                .names()
                .enumerate()
                .map(|(i, (name, is_active))| {
                    let text = Container::new(Text::new(name.to_string()).size(14))
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
                            .style(style::Button::Menu {
                                selected: is_active,
                            })
                            .on_press(Message::ProfilePressed(i))
                            .width(Length::Fill)
                            .padding(0)
                            .into()
                    }
                })
                .collect(),
        )
        .width(Length::Fill);

        let new_button = Button::new(Text::new("+ New Profile").size(14))
            .style(style::Button::Menu { selected: false })
            .on_press(Message::NewProfilePressed)
            .width(Length::Fill);

        Scrollable::new(Column::new().push(menu).push(new_button).width(175))
            .height(Length::Fill)
            .into()
    }
}

impl Screen {
    fn creating() -> Self {
        Self::Create {
            difficulty: Some(Difficulty::default()),

            layout: None,

            name_parsed: None,
            name_value: String::new(),
        }
    }

    fn renaming(name_value: String) -> Self {
        Self::Rename {
            name_parsed: None,
            name_value,
        }
    }

    fn viewing() -> Self {
        Self::View
    }

    fn view(&self, profiles: &profile::List) -> Element<Message> {
        let mut content = Column::new().width(Length::Fill).spacing(20);

        match self {
            Screen::Create {
                difficulty,
                layout,
                name_parsed,
                name_value,
            } => {
                let name_input = TextInput::new("Profile Name", name_value, Message::NameInput)
                    .width(Length::Fill)
                    .padding(6)
                    .size(18);

                let layout_title = Text::new("Keyboard Layout").size(14).font(font::Font::Thin);
                let layout_pick_list =
                    PickList::new(keyboard::ALL, *layout, Message::LayoutChanged).text_size(15);
                let layout_section = Column::new()
                    .spacing(5)
                    .push(layout_title)
                    .push(layout_pick_list);

                let difficulty_title = Text::new("Difficulty").size(14).font(font::Font::Thin);
                let difficulty_pick_list =
                    PickList::new(Difficulty::ALL, *difficulty, Message::DifficultyChanged)
                        .text_size(15);
                let difficulty_section = Column::new()
                    .spacing(5)
                    .push(difficulty_title)
                    .push(difficulty_pick_list);

                let mut accept =
                    Button::new(centered_text("\u{2714}", 24, 20)).style(style::Button::Accept);
                if name_parsed.is_some() && layout.is_some() {
                    accept = accept.on_press(Message::CreateAccept);
                }

                let cancel = Button::new(centered_text("\u{2716}", 24, 20))
                    .style(style::Button::Reject)
                    .on_press(Message::CreateCancel);

                let button_row = Row::new().push(accept).push(cancel).spacing(5);

                content = content
                    .push(name_input)
                    .push(layout_section)
                    .push(difficulty_section)
                    .push(button_row);
            }
            Screen::Rename {
                name_parsed,
                name_value,
            } => {
                let mut name_input = TextInput::new("Profile Name", name_value, Message::NameInput)
                    .width(Length::Fill)
                    .padding(6)
                    .size(18);

                let mut accept =
                    Button::new(centered_text("\u{2714}", 24, 20)).style(style::Button::Accept);

                if name_parsed.is_some() {
                    name_input = name_input.on_submit(Message::RenameAccept);
                    accept = accept.on_press(Message::RenameAccept);
                }

                let cancel = Button::new(centered_text("\u{2716}", 24, 20))
                    .style(style::Button::Reject)
                    .on_press(Message::RenameCancel);

                let name_row = Row::new()
                    .push(name_input)
                    .push(accept)
                    .push(cancel)
                    .spacing(5);

                content = content.push(name_row);
            }
            Screen::View => {
                let rename_button =
                    Button::new(Text::new(profiles.active().name.to_string()).size(18))
                        .style(style::Button::Text)
                        .on_press(Message::RenamePressed)
                        .padding(6);

                let layout_title = Text::new("Keyboard Layout").size(14).font(font::Font::Thin);
                let layout_name = Text::new(profiles.active().layout.to_string()).size(16);
                let layout_section = Column::new()
                    .padding([0, 0, 0, 6])
                    .spacing(5)
                    .push(layout_title)
                    .push(layout_name);

                let difficulty_title = Text::new("Difficulty").size(14).font(font::Font::Thin);
                let difficulty_pick_list = PickList::new(
                    Difficulty::ALL,
                    Some(profiles.active().difficulty),
                    Message::DifficultyChanged,
                )
                .text_size(15);
                let difficulty_section = Column::new()
                    .padding([0, 0, 0, 6])
                    .spacing(5)
                    .push(difficulty_title)
                    .push(difficulty_pick_list);

                content = content
                    .push(rename_button)
                    .push(layout_section)
                    .push(difficulty_section);
            }
        }

        content.into()
    }
}

fn centered_text(s: &str, size: u16, side: u16) -> Element<Message> {
    Container::new(Text::new(s).size(size))
        .width(side)
        .height(side)
        .center_x()
        .center_y()
        .into()
}
