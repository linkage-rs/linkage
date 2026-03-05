use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Alignment, Length, padding};

use crate::data::Theme;
use crate::{Element, style};

#[derive(Debug)]
pub struct State {
    themes: Vec<Theme>,
    active: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemePressed(usize),
}

pub enum Event {
    SelectTheme(Theme),
}

impl State {
    pub fn new(active: &str) -> Self {
        let themes = crate::data::theme::all();

        Self {
            themes,
            active: active.to_string(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::ThemePressed(index) => {
                if let Some(theme) = self.themes.get(index) {
                    self.active = theme.name().to_string();
                    return Some(Event::SelectTheme(theme.clone()));
                }
            }
        }
        None
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = container(text("Theme").size(18)).padding(6);

        let buttons = column(
            self.themes
                .iter()
                .enumerate()
                .map(|(i, theme)| {
                    let palette = theme.palette();

                    let target_color = palette.success;
                    let miss_color = palette.warning;
                    let error_color = palette.danger;
                    let text_color = palette.text;

                    let mut content = row![
                        text("\u{25a0}")
                            .size(18)
                            .class(style::Text::Override(target_color)),
                        text("\u{25a0}")
                            .size(18)
                            .class(style::Text::Override(miss_color)),
                        text("\u{25a0}")
                            .size(18)
                            .class(style::Text::Override(error_color)),
                        text(theme.name())
                            .size(16)
                            .class(style::Text::Override(text_color)),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center);

                    if self.active == theme.name() {
                        content = content
                            .push(Space::new().width(Length::Fill))
                            .push(text("\u{25cf}").size(18));
                    }

                    container(
                        button(content)
                            .on_press(Message::ThemePressed(i))
                            .class(style::Button::ThemePreview(theme.clone()))
                            .width(Length::Fill)
                            .padding([3, 7]),
                    )
                    .class(style::Container::theme_preview(theme))
                    .into()
                })
                .collect::<Vec<Element<Message>>>(),
        )
        .width(275)
        .spacing(7)
        .padding(padding::left(6));

        scrollable(
            column![title, buttons]
                .width(Length::Fill)
                .spacing(20)
                .padding(10),
        )
        .height(Length::Fill)
        .into()
    }
}
