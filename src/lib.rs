pub mod data;
pub mod font;
pub mod screen;
pub mod style;

pub type Element<'a, Message> = iced::Element<'a, Message, iced::Renderer<crate::style::Theme>>;
