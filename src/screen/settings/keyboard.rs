use iced::Length;
use iced::widget::{column, container, pick_list, row, scrollable, text, toggler};

use crate::data::Theme;
use crate::data::keyboard::{ALL_KEYBOARD_METRICS, KeyboardMetric};
use crate::data::profile;
use crate::screen::mini_keyboard::{MiniKeyboard, MiniKeyboardData};
use crate::{Element, font};

#[derive(Debug)]
pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {
    ShowToggled(bool),
    MetricChanged(KeyboardMetric),
}

impl State {
    pub fn new(_profiles: &profile::List) -> Self {
        Self {}
    }

    pub fn update(&mut self, profiles: &mut profile::List, message: Message) -> bool {
        match message {
            Message::ShowToggled(show) => {
                profiles.active_mut().mini_keyboard.show = show;
                true
            }
            Message::MetricChanged(metric) => {
                profiles.active_mut().mini_keyboard.metric = metric;
                true
            }
        }
    }

    pub fn view<'a>(&'a self, profiles: &profile::List, theme: &Theme) -> Element<'a, Message> {
        let settings = &profiles.active().mini_keyboard;
        let layout = &profiles.active().layout;

        let title = container(text("Mini Keyboard").size(18)).padding(6);

        let show_toggle = row![
            text("Show on training screen").size(14),
            toggler(settings.show)
                .on_toggle(Message::ShowToggled)
                .size(16),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center);

        let mut content = column![title, show_toggle].spacing(15).padding(10);

        let metric_title = text("Key Fill Metric")
            .size(14)
            .font(iced::Font::from(font::Font::Thin));
        let metric_pick = pick_list(
            ALL_KEYBOARD_METRICS,
            Some(settings.metric),
            Message::MetricChanged,
        )
        .text_size(15);
        let metric_section = column![metric_title, metric_pick].spacing(5);
        content = content.push(metric_section);

        // Preview
        let preview_title = text("Preview")
            .size(14)
            .font(iced::Font::from(font::Font::Thin));

        let preview_data = MiniKeyboardData::preview(layout, settings, theme);
        let preview_canvas = MiniKeyboard::view::<Message>(preview_data);
        let preview_container = container(preview_canvas).padding(10);

        content = content.push(preview_title).push(preview_container);

        scrollable(content.width(Length::Fill))
            .height(Length::Fill)
            .into()
    }
}
