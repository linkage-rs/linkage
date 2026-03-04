use iced::widget::container;
use iced::{Element, Length, Size, Subscription, Task, keyboard, window};
use linkage::data::{self, profile};
use linkage::screen::{self, Screen};
use linkage::{font, style};

pub fn main() -> iced::Result {
    iced::application(Linkage::new, Linkage::update, Linkage::view)
        .subscription(Linkage::subscription)
        .theme(Linkage::theme)
        .title("Linkage")
        .default_font(iced::Font::from(font::Font::Light))
        .window_size(Size::new(screen::training::OVERALL_WIDTH as f32, 600.0))
        .run()
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
    FontLoaded(Result<(), iced::font::Error>),
    Saved,
    Screen(screen::Message),
    WindowClose(window::Id),
    KeyboardEvent(keyboard::Event),
}

impl Linkage {
    fn new() -> (Linkage, Task<Message>) {
        let linkage = Linkage {
            screen: Screen::new(),
            theme: Default::default(),
            profiles: profile::List::default(),
        };
        (
            linkage,
            Task::batch(vec![
                iced::font::load(font::THIN).map(Message::FontLoaded),
                iced::font::load(font::LIGHT).map(Message::FontLoaded),
                iced::font::load(font::MEDIUM).map(Message::FontLoaded),
                Task::perform(screen::loading::load(), |message| {
                    Message::Screen(screen::Message::Loading(message))
                }),
            ]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(Ok(_)) => Task::none(),
            Message::FontLoaded(Err(_)) => {
                eprintln!("Could not load font.");
                Task::none()
            }
            Message::Saved => Task::none(),
            Message::WindowClose(_id) => self.prepare_close(),
            Message::KeyboardEvent(event) => self.handle_keyboard_event(event),
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
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let Linkage {
            screen, profiles, ..
        } = self;
        let content = screen.view(profiles).map(Message::Screen);
        let theme_data = self.theme.data().clone();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(style::Container::Primary.style_fn(&theme_data))
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.screen.subscription().map(Message::Screen),
            window::close_requests().map(Message::WindowClose),
            keyboard::listen().map(Message::KeyboardEvent),
        ])
    }

    fn theme(&self) -> iced::Theme {
        // We use the built-in theme as a base but override via style closures
        iced::Theme::custom(
            "Linkage".to_string(),
            iced::theme::Palette {
                background: self.theme.data().bg,
                text: self.theme.data().text,
                primary: self.theme.data().target,
                success: self.theme.data().target,
                danger: self.theme.data().error,
                warning: self.theme.data().miss,
            },
        )
    }

    fn handle_keyboard_event(&mut self, event: keyboard::Event) -> Task<Message> {
        use iced::keyboard::Key;
        use iced::keyboard::key::Named;

        match event {
            keyboard::Event::KeyPressed { key, modifiers, .. } => match key.as_ref() {
                Key::Named(Named::Escape) => {
                    return self.go_back();
                }
                #[cfg(target_os = "macos")]
                Key::Character("q") if modifiers.command() => {
                    return self.prepare_close();
                }
                _ => {}
            },
            _ => {}
        }

        Task::none()
    }

    fn prepare_close(&mut self) -> Task<Message> {
        println!("Preparing to close.");
        iced::exit()
    }

    fn go_back(&mut self) -> Task<Message> {
        self.screen.go_back(&self.profiles);
        Task::none()
    }

    fn save(&self) -> Task<Message> {
        let saved = data::Saved::new(self.profiles.clone(), self.theme.name());
        Task::perform(save(saved), |_| Message::Saved)
    }
}

async fn save(saved: data::Saved) -> bool {
    saved.save().await.is_ok()
}
