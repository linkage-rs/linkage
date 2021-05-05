use iced::executor;
use iced::{
    self, Application, Clipboard, Command, Container, Element, Settings, Subscription, Text,
};
use iced_native;

mod data;

pub fn main() -> iced::Result {
    let freq = data::freq::Freq::load();

    dbg!(&freq);

    Linkage::run(Settings {
        flags: Flags {
            freq: freq.unwrap_or_default(),
        },
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct Linkage {
    should_exit: bool,
    freq: data::freq::Freq,
}

#[derive(Debug, Clone)]
enum Message {
    Event(iced_native::Event),
    Exit,
}

#[derive(Debug, Default)]
struct Flags {
    freq: data::freq::Freq,
}

impl Application for Linkage {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Flags) -> (Linkage, Command<Message>) {
        let linkage = Linkage {
            should_exit: false,
            freq: flags.freq,
        };
        (linkage, Command::none())
    }

    fn title(&self) -> String {
        String::from("Linkage")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Event(event) => {
                println!("{:?}", event);
            }
            Message::Exit => {
                self.should_exit = true;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::Event)
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn view(&mut self) -> Element<Message> {
        Container::new(Text::new("Linkage"))
            .center_x()
            .center_y()
            .into()
    }
}
