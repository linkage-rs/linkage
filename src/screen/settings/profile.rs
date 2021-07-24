use crate::data::keyboard::{self, Layout};
use crate::data::training::Difficulty;
use crate::data::{self, Theme};
use crate::font;
use crate::style;
use iced::button::{self, Button};
use iced::pick_list::{self, PickList};
use iced::scrollable::{self, Scrollable};
use iced::text_input::{self, TextInput};
use iced::{Column, Container, Element, Length, Row, Rule, Text};

#[derive(Debug)]
pub struct State {
    menu: Menu,
    content_scroll: scrollable::State,
    screen: Screen,
}

#[derive(Debug)]
pub struct Menu {
    buttons: Vec<button::State>,
    new_button: button::State,
    scroll: scrollable::State,
}

#[derive(Debug)]
pub enum Screen {
    Create {
        accept: button::State,
        cancel: button::State,
        difficulty: Option<Difficulty>,
        difficulty_pick_list: pick_list::State<Difficulty>,
        layout: Option<Layout>,
        layout_pick_list: pick_list::State<Layout>,
        name_input: text_input::State,
        name_parsed: Option<data::profile::Name>,
        name_value: String,
    },
    Rename {
        accept: button::State,
        cancel: button::State,
        name_input: text_input::State,
        name_parsed: Option<data::profile::Name>,
        name_value: String,
    },
    View {
        difficulty_pick_list: pick_list::State<Difficulty>,
        rename_button: button::State,
        // delete_button
    },
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
            content_scroll: scrollable::State::new(),
            screen: Screen::viewing(),
        }
    }

    pub fn update(&mut self, profiles: &mut data::profile::List, message: Message) -> bool {
        match message {
            Message::CreateAccept => {
                if let Screen::Create {
                    difficulty: Some(difficulty),
                    layout: Some(layout),
                    name_parsed: Some(name_parsed),
                    ..
                } = &self.screen
                {
                    let profile =
                        data::profile::Profile::new(name_parsed.clone(), *layout, *difficulty);
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
                    if let Some(name) = data::profile::Name::parse(&new_name) {
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

    pub fn view(&mut self, profiles: &data::profile::List, theme: &Theme) -> Element<Message> {
        let State {
            menu,
            content_scroll,
            screen,
        } = self;

        let menu = menu.view(profiles, theme);

        let content = Scrollable::new(content_scroll)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .push(screen.view(profiles, theme));

        Row::new()
            .push(menu)
            .push(Rule::vertical(0).style(style::rule::divider(theme)))
            .push(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl Menu {
    fn new() -> Self {
        Self {
            buttons: Vec::new(),
            new_button: button::State::new(),
            scroll: scrollable::State::new(),
        }
    }

    fn view(&mut self, profiles: &data::profile::List, theme: &Theme) -> Element<Message> {
        let Menu {
            buttons,
            new_button,
            scroll,
        } = self;

        buttons.resize(profiles.len(), button::State::new());

        let menu = Column::with_children(
            profiles
                .names()
                .zip(buttons.iter_mut())
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

        Scrollable::new(scroll)
            .push(menu)
            .push(new_button)
            .height(Length::Fill)
            .width(Length::Units(175))
            .into()
    }
}

impl Screen {
    fn creating() -> Self {
        Self::Create {
            accept: button::State::new(),
            cancel: button::State::new(),
            difficulty: Some(Difficulty::default()),
            difficulty_pick_list: pick_list::State::default(),
            layout: None,
            layout_pick_list: pick_list::State::default(),
            name_input: text_input::State::new(),
            name_parsed: None,
            name_value: String::new(),
        }
    }

    fn renaming(name_value: String) -> Self {
        Self::Rename {
            accept: button::State::new(),
            cancel: button::State::new(),
            name_input: text_input::State::new(),
            name_parsed: None,
            name_value,
        }
    }

    fn viewing() -> Self {
        Self::View {
            difficulty_pick_list: pick_list::State::default(),
            rename_button: button::State::new(),
        }
    }

    fn view(&mut self, profiles: &data::profile::List, theme: &Theme) -> Element<Message> {
        let mut content = Column::new().width(Length::Fill).spacing(20);

        match self {
            Screen::Create {
                accept,
                cancel,
                difficulty,
                difficulty_pick_list,
                layout,
                layout_pick_list,
                name_input,
                name_parsed,
                name_value,
            } => {
                let name_input =
                    TextInput::new(name_input, "Profile Name", name_value, Message::NameInput)
                        .style(style::text_input::themed(theme))
                        .width(Length::Fill)
                        .padding(6)
                        .size(18);

                let layout_title = Text::new("Keyboard Layout").size(14).font(font::THIN);
                let layout_pick_list = PickList::new(
                    layout_pick_list,
                    keyboard::ALL,
                    *layout,
                    Message::LayoutChanged,
                )
                .text_size(15)
                .style(style::pick_list::themed(theme));
                let layout_section = Column::new()
                    .spacing(5)
                    .push(layout_title)
                    .push(layout_pick_list);

                let difficulty_title = Text::new("Difficulty").size(14).font(font::THIN);
                let difficulty_pick_list = PickList::new(
                    difficulty_pick_list,
                    Difficulty::ALL,
                    *difficulty,
                    Message::DifficultyChanged,
                )
                .text_size(15)
                .style(style::pick_list::themed(theme));
                let difficulty_section = Column::new()
                    .spacing(5)
                    .push(difficulty_title)
                    .push(difficulty_pick_list);

                let mut accept = Button::new(accept, centered_text("\u{2714}", 20, 20))
                    .style(style::button::accept(theme));
                if name_parsed.is_some() && layout.is_some() {
                    accept = accept.on_press(Message::CreateAccept);
                }

                let cancel = Button::new(cancel, centered_text("\u{2716}", 20, 20))
                    .style(style::button::reject(theme))
                    .on_press(Message::CreateCancel);

                let button_row = Row::new().push(accept).push(cancel).spacing(5);

                content = content
                    .push(name_input)
                    .push(layout_section)
                    .push(difficulty_section)
                    .push(button_row);
            }
            Screen::Rename {
                accept,
                cancel,
                name_input,
                name_parsed,
                name_value,
            } => {
                let mut name_input =
                    TextInput::new(name_input, "Profile Name", name_value, Message::NameInput)
                        .style(style::text_input::themed(theme))
                        .width(Length::Fill)
                        .padding(6)
                        .size(18);

                let mut accept = Button::new(accept, centered_text("\u{2714}", 20, 20))
                    .style(style::button::accept(theme));

                if name_parsed.is_some() {
                    name_input = name_input.on_submit(Message::RenameAccept);
                    accept = accept.on_press(Message::RenameAccept);
                }

                let cancel = Button::new(cancel, centered_text("\u{2716}", 20, 20))
                    .style(style::button::reject(theme))
                    .on_press(Message::RenameCancel);

                let name_row = Row::new()
                    .push(name_input)
                    .push(accept)
                    .push(cancel)
                    .spacing(5);

                content = content.push(name_row);
            }
            Screen::View {
                difficulty_pick_list,
                rename_button,
            } => {
                let rename_button = Button::new(
                    rename_button,
                    Text::new(profiles.active().name.to_string()).size(18),
                )
                .style(style::button::text(theme))
                .on_press(Message::RenamePressed)
                .padding(6);

                let layout_title = Text::new("Keyboard Layout").size(14).font(font::THIN);
                let layout_name = Text::new(profiles.active().layout.to_string()).size(16);
                let layout_section = Column::new()
                    .padding([0, 0, 0, 6])
                    .spacing(5)
                    .push(layout_title)
                    .push(layout_name);

                let difficulty_title = Text::new("Difficulty").size(14).font(font::THIN);
                let difficulty_pick_list = PickList::new(
                    difficulty_pick_list,
                    Difficulty::ALL,
                    Some(profiles.active().difficulty),
                    Message::DifficultyChanged,
                )
                .text_size(15)
                .style(style::pick_list::themed(theme));
                let difficulty_section = Column::new()
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

fn centered_text(s: &str, size: u16, side: u16) -> Container<Message> {
    Container::new(Text::new(s).size(size))
        .width(Length::Units(side))
        .height(Length::Units(side))
        .center_x()
        .center_y()
}
