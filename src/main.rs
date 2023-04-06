use iced::executor;
use iced::widget::Container;
use iced::window;
use iced::{self, Application, Command, Element, Length, Settings, Subscription};

use linkage::data::{self, profile};
use linkage::screen::{self, Screen};
use linkage::{font, style};

pub fn main() -> iced::Result {
    Linkage::run(Settings {
        default_font: font::Font::Light.into(),
        exit_on_close_request: false,
        window: iced::window::Settings {
            min_size: Some((screen::training::OVERALL_WIDTH as u32, 256)),
            ..Default::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
struct Linkage {
    screen: Screen,
    theme: style::Theme,
    profiles: profile::List,
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
enum Message {
    Event(iced::Event),
    FontLoaded(Result<(), iced::font::Error>),
    Saved,
    Screen(screen::Message),
}

impl Application for Linkage {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = style::Theme;
    type Flags = ();

    fn new(_: ()) -> (Linkage, Command<Message>) {
        let linkage = Linkage {
            screen: Screen::new(),
            theme: Default::default(),
            profiles: profile::List::default(),
        };
        (
            linkage,
            Command::batch(vec![
                iced::font::load(font::THIN).map(Message::FontLoaded),
                iced::font::load(font::LIGHT).map(Message::FontLoaded),
                iced::font::load(font::MEDIUM).map(Message::FontLoaded),
                Command::perform(screen::loading::load(), |message| {
                    Message::Screen(screen::Message::Loading(message))
                }),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Linkage")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Event(event) => self.handle_event(event),
            Message::FontLoaded(Ok(_)) => Command::none(),
            Message::FontLoaded(Err(_)) => {
                eprintln!("Could not load font.");
                Command::none()
            }
            Message::Saved => Command::none(),
            Message::Screen(message) => {
                let Linkage {
                    screen,
                    theme,
                    profiles,
                    ..
                } = self;
                if let Some(event) = screen.update(profiles, message, theme.name()) {
                    match event {
                        screen::Event::ExitRequested => self.prepare_close(),
                        screen::Event::Save => self.save(),
                        screen::Event::SelectTheme(new_theme) => {
                            *theme = style::Theme::new(new_theme);
                            self.save()
                        }
                    }
                } else {
                    Command::none()
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.screen.subscription().map(Message::Screen),
            iced::subscription::events().map(Message::Event),
        ])
    }

    fn view(&self) -> Element<Self::Message, iced::Renderer<Self::Theme>> {
        let Linkage {
            screen, profiles, ..
        } = self;
        let content = screen.view(profiles).map(Message::Screen);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Container::Primary)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}

impl Linkage {
    fn handle_event(&mut self, event: iced::Event) -> Command<Message> {
        use iced::keyboard::{self, KeyCode};
        use iced::Event;

        match event {
            Event::Window(window::Event::CloseRequested) => {
                return self.prepare_close();
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code,
                modifiers,
            }) => match key_code {
                KeyCode::Escape => {
                    return self.go_back();
                }
                #[cfg(target_os = "macos")]
                KeyCode::Q if modifiers.command() => {
                    return self.prepare_close();
                }
                _ => {}
            },
            _ => {}
        }

        Command::none()
    }

    fn prepare_close(&mut self) -> Command<Message> {
        use iced::widget::runtime::command;

        println!("Preparing to close.");
        Command::single(command::Action::Window(window::Action::Close))
    }

    fn go_back(&mut self) -> Command<Message> {
        self.screen.go_back(&self.profiles);
        Command::none()
    }

    fn save(&self) -> Command<Message> {
        let saved = data::Saved::new(self.profiles.clone(), self.theme.name());
        Command::perform(save(saved), |_| Message::Saved)
    }
}

async fn save(saved: data::Saved) -> bool {
    saved.save().await.is_ok()
}
