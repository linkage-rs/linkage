use iced::executor;
use iced::{
    self, Application, Clipboard, Command, Container, Element, Length, Settings, Subscription,
};
use iced_native;
use iced_native::window;

mod data;
mod font;
mod screen;
mod style;

use data::profile;
use data::Theme;
use screen::Screen;

pub fn main() -> iced::Result {
    let default_font = if let iced::Font::External { bytes, .. } = font::LIGHT {
        Some(bytes)
    } else {
        None
    };

    Linkage::run(Settings {
        default_font,
        exit_on_close_request: false,
        ..Settings::default()
    })
}

#[derive(Debug)]
struct Linkage {
    should_exit: bool,
    screen: Screen,
    theme: Theme,
    profiles: profile::List,
}

#[derive(Debug, Clone)]
enum Message {
    Event(iced_native::Event),
    Saved,
    Screen(screen::Message),
}

impl Application for Linkage {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: ()) -> (Linkage, Command<Message>) {
        let linkage = Linkage {
            should_exit: false,
            screen: Screen::new(),
            theme: Theme::monokai(),
            profiles: profile::List::default(),
        };
        (
            linkage,
            Command::perform(screen::loading::load(), |message| {
                Message::Screen(screen::Message::Loading(message))
            }),
        )
    }

    fn title(&self) -> String {
        String::from("Linkage")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Event(event) => self.handle_event(event),
            Message::Saved => Command::none(),
            Message::Screen(message) => {
                let Linkage {
                    screen,
                    theme,
                    profiles,
                    ..
                } = self;
                if let Some(event) = screen.update(profiles, message) {
                    match event {
                        screen::Event::ExitRequested => {
                            self.prepare_close();
                            Command::none()
                        }
                        screen::Event::Save => self.save(),
                        screen::Event::SelectTheme(new_theme) => {
                            *theme = new_theme;
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
            iced_native::subscription::events().map(Message::Event),
        ])
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn view(&mut self) -> Element<Message> {
        let Linkage {
            screen,
            theme,
            profiles,
            ..
        } = self;
        let content = screen.view(profiles, theme).map(Message::Screen);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::container::primary(&self.theme))
            .into()
    }
}

impl Linkage {
    fn handle_event(&mut self, event: iced_native::Event) -> Command<Message> {
        use iced::keyboard::{self, KeyCode};
        use iced_native::event::Event;

        match event {
            Event::Window(window::Event::CloseRequested) => {
                return self.prepare_close();
            }
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                } => match key_code {
                    KeyCode::Escape => {
                        return self.go_back();
                    }
                    #[cfg(target_os = "macos")]
                    KeyCode::Q if modifiers.is_command_pressed() => {
                        return self.prepare_close();
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        Command::none()
    }

    fn prepare_close(&mut self) -> Command<Message> {
        println!("Preparing to close.");
        self.should_exit = true;
        Command::none()
    }

    fn go_back(&mut self) -> Command<Message> {
        self.screen.go_back(&self.profiles);
        Command::none()
    }

    fn save(&self) -> Command<Message> {
        let saved = data::Saved::new(self.profiles.clone(), &self.theme);
        Command::perform(save(saved), |_| Message::Saved)
    }
}

async fn save(saved: data::Saved) -> bool {
    saved.save().await.is_ok()
}
