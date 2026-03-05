use iced::widget::{
    Column, Row, button, column, container, pick_list, row, rule, scrollable, text, text_input,
};
use iced::{Length, padding};

use crate::data::keyboard::{self, Layout};
use crate::data::profile;
use crate::data::training::Difficulty;
use crate::{Element, font, style};

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
            Message::DifficultyChanged(new_difficulty) => match &mut self.screen {
                Screen::Create { difficulty, .. } => {
                    *difficulty = Some(new_difficulty);
                }
                Screen::View => {
                    profiles.active_mut().difficulty = new_difficulty;

                    return true;
                }
                _ => {}
            },
            Message::LayoutChanged(new_layout) => {
                if let Screen::Create { layout, .. } = &mut self.screen {
                    *layout = Some(new_layout);
                }
            }
            Message::NameInput(new_name) => match &mut self.screen {
                Screen::Create {
                    name_parsed,
                    name_value,
                    ..
                }
                | Screen::Rename {
                    name_parsed,
                    name_value,
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

    pub fn view(&self, profiles: &profile::List) -> Element<'_, Message> {
        let State { menu, screen } = self;

        let menu = menu.view(profiles);

        let content = scrollable(
            container(screen.view(profiles))
                .width(Length::Fill)
                .padding(10),
        )
        .height(Length::Fill);

        Row::new()
            .push(menu)
            .push(rule::vertical(0))
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

    fn view(&self, profiles: &profile::List) -> Element<'_, Message> {
        let menu = Column::with_children(
            profiles
                .names()
                .enumerate()
                .map(|(i, (name, is_active))| -> Element<Message> {
                    let label = container(text(name.to_string()).size(14))
                        .padding(6)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill);
                    if is_active {
                        container(label)
                            .class(style::Container::RoundedBox)
                            .width(Length::Fill)
                            .into()
                    } else {
                        button(label)
                            .class(style::Button::Text)
                            .on_press(Message::ProfilePressed(i))
                            .width(Length::Fill)
                            .padding(0)
                            .into()
                    }
                })
                .collect::<Vec<Element<Message>>>(),
        )
        .width(Length::Fill);

        let new_button = button(text("+ New Profile").size(14))
            .class(style::Button::Text)
            .on_press(Message::NewProfilePressed)
            .width(Length::Fill);

        scrollable(column![menu, new_button].width(175))
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

    fn view(&self, profiles: &profile::List) -> Element<'_, Message> {
        let mut content = Column::new().width(Length::Fill).spacing(20);

        match self {
            Screen::Create {
                difficulty,
                layout,
                name_parsed,
                name_value,
            } => {
                let name_input = text_input("Profile Name", name_value)
                    .on_input(Message::NameInput)
                    .width(Length::Fill)
                    .padding(6)
                    .size(18);

                let layout_title = text("Keyboard Layout")
                    .size(14)
                    .font(iced::Font::from(font::Font::Thin));
                let layout_pick_list =
                    pick_list(keyboard::ALL, *layout, Message::LayoutChanged).text_size(15);
                let layout_section = column![layout_title, layout_pick_list].spacing(5);

                let difficulty_title = text("Difficulty")
                    .size(14)
                    .font(iced::Font::from(font::Font::Thin));
                let difficulty_pick_list =
                    pick_list(Difficulty::ALL, *difficulty, Message::DifficultyChanged)
                        .text_size(15);
                let difficulty_section = column![difficulty_title, difficulty_pick_list].spacing(5);

                let mut accept =
                    button(centered_text("\u{2714}", 24, 20)).class(style::Button::Accept);
                if name_parsed.is_some() && layout.is_some() {
                    accept = accept.on_press(Message::CreateAccept);
                }

                let cancel = button(centered_text("\u{2716}", 24, 20))
                    .class(style::Button::Reject)
                    .on_press(Message::CreateCancel);

                let button_row = row![accept, cancel].spacing(5);

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
                let mut name_input = text_input("Profile Name", name_value)
                    .class(style::TextInput)
                    .on_input(Message::NameInput)
                    .width(Length::Fill)
                    .padding(6)
                    .size(18);

                let mut accept =
                    button(centered_text("\u{2714}", 24, 20)).class(style::Button::Accept);

                if name_parsed.is_some() {
                    name_input = name_input.on_submit(Message::RenameAccept);
                    accept = accept.on_press(Message::RenameAccept);
                }

                let cancel = button(centered_text("\u{2716}", 24, 20))
                    .class(style::Button::Reject)
                    .on_press(Message::RenameCancel);

                let name_row = row![name_input, accept, cancel].spacing(5);

                content = content.push(name_row);
            }
            Screen::View => {
                let rename_button = button(text(profiles.active().name.to_string()).size(18))
                    .class(style::Button::Text)
                    .on_press(Message::RenamePressed)
                    .padding(6);

                let layout_title = text("Keyboard Layout")
                    .size(14)
                    .font(iced::Font::from(font::Font::Thin));
                let layout_name = text(profiles.active().layout.to_string()).size(16);
                let layout_section = column![layout_title, layout_name]
                    .padding(padding::left(6))
                    .spacing(5);

                let difficulty_title = text("Difficulty")
                    .size(14)
                    .font(iced::Font::from(font::Font::Thin));
                let difficulty_pick_list = pick_list(
                    Difficulty::ALL,
                    Some(profiles.active().difficulty),
                    Message::DifficultyChanged,
                )
                .class(style::PickList)
                .text_size(15);
                let difficulty_section = column![difficulty_title, difficulty_pick_list]
                    .padding(padding::left(6))
                    .spacing(5);

                content = content
                    .push(rename_button)
                    .push(layout_section)
                    .push(difficulty_section);
            }
        }

        content.into()
    }
}

fn centered_text(s: &str, size: u16, side: u16) -> Element<'_, Message> {
    let side_f = side as f32;
    let size_f = size as f32;
    container(text(s.to_string()).size(size_f))
        .center_x(side_f)
        .center_y(side_f)
        .into()
}
