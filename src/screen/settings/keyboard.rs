use iced::Length;
use iced::widget::{column, container, pick_list, scrollable, text};

use crate::data::Theme;
use crate::data::keyboard::{Metric, Size};
use crate::data::profile;
use crate::screen::mini_keyboard::{MiniKeyboard, MiniKeyboardData};
use crate::{Element, font};

#[derive(Debug)]
pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {
    MetricChanged(Metric),
    SizeChanged(Size),
}

impl State {
    pub fn new(_profiles: &profile::List) -> Self {
        Self {}
    }

    pub fn update(&mut self, profiles: &mut profile::List, message: Message) {
        match message {
            Message::MetricChanged(metric) => {
                profiles.active_mut().mini_keyboard.metric = metric;
            }
            Message::SizeChanged(size) => {
                profiles.active_mut().mini_keyboard.size = size;
            }
        }
    }

    pub fn view<'a>(&'a self, profiles: &profile::List, theme: &Theme) -> Element<'a, Message> {
        let settings = &profiles.active().mini_keyboard;
        let layout = &profiles.active().layout;

        let title = container(text("Mini Keyboard").size(18)).padding([6, 0]);

        let size_title = text("Mini Keyboard Size")
            .size(14)
            .font(iced::Font::from(font::Font::Thin));
        let size_pick =
            pick_list(Size::ALL, Some(settings.size), Message::SizeChanged).text_size(15);
        let size_section = column![size_title, size_pick].spacing(5);

        let mut content = column![title, size_section].spacing(15).padding(10);

        if settings.size.is_visible() {
            let metric_title = text("Key Fill Metric")
                .size(14)
                .font(iced::Font::from(font::Font::Thin));
            let metric_pick =
                pick_list(Metric::ALL, Some(settings.metric), Message::MetricChanged).text_size(15);
            let metric_section = column![metric_title, metric_pick].spacing(5);
            content = content.push(metric_section);

            let preview_title = text("Preview")
                .size(14)
                .font(iced::Font::from(font::Font::Thin));

            let preview_data = MiniKeyboardData::preview(layout, settings, theme);
            let preview_canvas = MiniKeyboard::view::<Message>(preview_data);
            let preview_container = container(preview_canvas).padding(10);

            content = content.push(preview_title).push(preview_container);
        }

        scrollable(content.width(Length::Fill))
            .height(Length::Fill)
            .into()
    }
}
