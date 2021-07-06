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

use data::user;
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
    users: user::List,
}

#[derive(Debug, Clone)]
enum Message {
    Event(iced_native::Event),
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
            users: user::List::default(),
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
            Message::Screen(message) => {
                let Linkage { screen, users, .. } = self;
                if let Some((command, event)) = screen.update(users, message) {
                    match event {
                        screen::Event::ExitRequested => {
                            Command::batch(vec![command.map(Message::Screen), self.prepare_close()])
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
            users,
            ..
        } = self;
        let content = screen.view(users, theme).map(Message::Screen);

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
        match event {
            iced_native::Event::Window(window::Event::CloseRequested) => self.prepare_close(),
            _ => Command::none(),
        }
    }

    fn prepare_close(&mut self) -> Command<Message> {
        println!("Preparing to close.");
        self.should_exit = true;
        Command::none()
    }
}
