use iced::executor;
use iced::keyboard;
use iced::{
    self, Application, Clipboard, Command, Container, Element, Length, Settings, Subscription,
};
use iced_native;
use iced_native::window;

mod data;
mod screen;

use data::{Freq, User};
use screen::Screen;

pub fn main() -> iced::Result {
    let freq = Freq::load();

    Linkage::run(Settings {
        flags: Flags {
            freq: freq.unwrap_or_default(),
        },
        exit_on_close_request: false,
        ..Settings::default()
    })
}

#[derive(Debug)]
struct Linkage {
    should_exit: bool,
    freq: Freq,
    screen: Screen,
}

#[derive(Debug, Clone)]
enum Message {
    Event(iced_native::Event),
    Screen(screen::Message),
}

#[derive(Debug, Default)]
struct Flags {
    freq: Freq,
}

impl Application for Linkage {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Flags) -> (Linkage, Command<Message>) {
        let linkage = Linkage {
            should_exit: false,
            freq: flags.freq,
            screen: Screen::new(),
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
                if let Some((command, event)) = self.screen.update(message) {
                    match event {
                        screen::Event::ExitRequested => {
                            Command::batch(vec![command.map(Message::Screen), self.prepare_close()])
                        }
                        screen::Event::Training(user) => {
                            self.screen = Screen::training(user);
                            Command::none()
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
        let content = self.screen.view().map(Message::Screen);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
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
        println!("{:?}", &self.screen);
        self.should_exit = true;
        Command::none()
    }
}
