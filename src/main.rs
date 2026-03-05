use std::time::{Duration, Instant};

use iced::widget::{column, container, text};
use iced::{Length, Size, Subscription, Task, keyboard, window};
use linkage::data::{self, Theme, profile};
use linkage::screen::{self, Screen};
use linkage::{Element, font, style};

pub fn main() -> iced::Result {
    iced::application(Linkage::new, Linkage::update, Linkage::view)
        .subscription(Linkage::subscription)
        .theme(Linkage::theme)
        .title("Linkage")
        .default_font(iced::Font::from(font::Font::Light))
        .window_size(Size::new(
            screen::training::OVERALL_WIDTH as f32 + 100.0,
            600.0,
        ))
        .run()
}

/// Duration a toast stays visible.
const TOAST_DURATION: Duration = Duration::from_secs(5);
/// Maximum number of simultaneous toasts.
const MAX_TOASTS: usize = 5;

#[derive(Debug, Clone)]
struct Toast {
    message: String,
    created: Instant,
}

#[derive(Debug)]
struct Linkage {
    screen: Screen,
    theme: Theme,
    profiles: profile::List,
    toasts: Vec<Toast>,
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
enum Message {
    FontLoaded(Result<(), iced::font::Error>),
    Saved(Result<(), String>),
    Screen(screen::Message),
    WindowClose(window::Id),
    KeyboardEvent(keyboard::Event),
    ToastTick,
}

impl Linkage {
    fn new() -> (Linkage, Task<Message>) {
        let linkage = Linkage {
            screen: Screen::new(),
            theme: Default::default(),
            profiles: profile::List::default(),
            toasts: Vec::new(),
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

    fn add_toast(&mut self, message: String) {
        eprintln!("Toast: {}", message);
        self.toasts.push(Toast {
            message,
            created: Instant::now(),
        });
        while self.toasts.len() > MAX_TOASTS {
            self.toasts.remove(0);
        }
    }

    fn expire_toasts(&mut self) {
        let now = Instant::now();
        self.toasts
            .retain(|t| now.duration_since(t.created) < TOAST_DURATION);
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(Ok(_)) => Task::none(),
            Message::FontLoaded(Err(_)) => {
                self.add_toast("Could not load font.".to_string());
                Task::none()
            }
            Message::Saved(Ok(())) => Task::none(),
            Message::Saved(Err(err)) => {
                self.add_toast(format!("Save failed: {}", err));
                Task::none()
            }
            Message::WindowClose(_id) => self.prepare_close(),
            Message::KeyboardEvent(event) => self.handle_keyboard_event(event),
            Message::ToastTick => {
                self.expire_toasts();
                Task::none()
            }
            Message::Screen(message) => {
                let Linkage {
                    screen,
                    theme,
                    profiles,
                    ..
                } = self;
                let events = screen.update(profiles, message, theme.name());
                if events.is_empty() {
                    return Task::none();
                }
                let mut tasks = Vec::new();
                for event in events {
                    match event {
                        screen::Event::ExitRequested => {
                            return self.prepare_close();
                        }
                        screen::Event::Save => {
                            tasks.push(self.save());
                        }
                        screen::Event::SelectTheme(new_theme) => {
                            self.theme = new_theme;
                            tasks.push(self.save());
                        }
                        screen::Event::Toast(msg) => {
                            self.add_toast(msg);
                        }
                    }
                }
                Task::batch(tasks)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let Linkage {
            screen,
            profiles,
            theme,
            ..
        } = self;
        let content = screen.view(profiles, theme).map(Message::Screen);

        let base = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .class(style::Container::Primary);

        if self.toasts.is_empty() {
            base.into()
        } else {
            // Build toast overlay column
            let toast_col = column(
                self.toasts
                    .iter()
                    .map(|t| {
                        container(text(t.message.clone()).size(12).class(style::Text::Error))
                            .class(style::Container::Toast)
                            .padding([4, 8])
                            .max_width(300)
                            .into()
                    })
                    .collect::<Vec<Element<Message>>>(),
            )
            .spacing(4)
            .align_x(iced::Alignment::End);

            let toast_container = container(toast_col)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Right)
                .align_y(iced::alignment::Vertical::Top)
                .padding([8, 8]);

            // Stack using a simple column with the toasts overlaid.
            // We use iced::widget::stack for z-ordering.
            iced::widget::stack![base, toast_container]
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![
            self.screen.subscription().map(Message::Screen),
            window::close_requests().map(Message::WindowClose),
            keyboard::listen().map(Message::KeyboardEvent),
        ];

        if !self.toasts.is_empty() {
            subs.push(iced::time::every(Duration::from_millis(500)).map(|_| Message::ToastTick));
        }

        Subscription::batch(subs)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
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
        Task::perform(save(saved), Message::Saved)
    }
}

async fn save(saved: data::Saved) -> Result<(), String> {
    saved.save().await.map_err(|e| e.to_string())
}
