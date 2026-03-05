use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Color, Length, Rectangle, Renderer, Size};
use iced::{Point, mouse};

use crate::data::CharSet;
use crate::data::Theme;
use crate::data::keyboard::{KeyboardMetric, Layout, MiniKeyboardSettings, PhysicalKey};
use crate::data::training::{State as TrainingState, TriplePoint};

/// Pixel size of one key unit (internal width/height).
const KEY_SIZE: f32 = 15.0;
/// Stroke width for key outlines.
const STROKE_WIDTH: f32 = 1.0;

/// Data needed to render the mini keyboard in a single frame.
#[derive(Debug, Clone)]
pub struct MiniKeyboardData {
    /// Physical key positions for the layout.
    pub keys: Vec<PhysicalKey>,
    /// Total keyboard size in key units (width, height).
    pub board_size: (f32, f32),
    /// Set of characters the user has unlocked.
    pub unlocked: CharSet,
    /// Set of characters currently pressed (held down).
    pub pressed: HashSet<char>,
    /// Which metric to use for fill.
    pub metric: KeyboardMetric,
    /// Theme colors.
    pub text_color: Color,
    pub primary_color: Color,
    /// Per-character metric fill colors (pre-computed from theme).
    pub metric_colors: HashMap<char, Color>,
}

impl MiniKeyboardData {
    pub fn new(
        layout: &Layout,
        settings: &MiniKeyboardSettings,
        training_state: &TrainingState,
        accuracy_metric: &TriplePoint,
        wpm_metric: &TriplePoint,
        theme: &Theme,
        pressed: &HashSet<char>,
    ) -> Self {
        let keys = layout.physical_keys();
        let board_size = layout.keyboard_size();

        let unlocked: CharSet = training_state
            .clean_letters()
            .iter()
            .map(|(ch, _)| *ch)
            .collect();

        let mut metric_colors = HashMap::new();

        for (ch, clean_val) in training_state.clean_letters() {
            if ch == ' ' {
                continue;
            }
            let fill_value = match settings.metric {
                KeyboardMetric::Accuracy => accuracy_metric.value(clean_val),
                KeyboardMetric::Speed => {
                    if let Some(stats) = training_state.timings.get(&ch) {
                        let wpm: f32 = stats.wpm_harmonic_mean.into();
                        wpm_metric.value(wpm)
                    } else {
                        0.0
                    }
                }
            };

            let color = theme.metric(fill_value);
            metric_colors.insert(ch, color);
        }

        Self {
            keys,
            board_size,
            unlocked,
            pressed: pressed.clone(),
            metric: settings.metric,
            text_color: theme.text(),
            primary_color: theme.palette().primary,
            metric_colors,
        }
    }

    /// Build a simpler version for settings preview (no training state).
    pub fn preview(layout: &Layout, settings: &MiniKeyboardSettings, theme: &Theme) -> Self {
        let keys = layout.physical_keys();
        let board_size = layout.keyboard_size();

        Self {
            keys,
            board_size,
            unlocked: HashSet::new(),
            pressed: HashSet::new(),
            metric: settings.metric,
            text_color: theme.text(),
            primary_color: theme.palette().primary,
            metric_colors: HashMap::new(),
        }
    }

    /// The pixel width of the rendered keyboard.
    pub fn pixel_width(&self) -> f32 {
        self.board_size.0 * (KEY_SIZE + STROKE_WIDTH) + STROKE_WIDTH
    }

    /// The pixel height of the rendered keyboard.
    pub fn pixel_height(&self) -> f32 {
        self.board_size.1 * (KEY_SIZE + STROKE_WIDTH) + STROKE_WIDTH
    }
}

/// Draw the mini keyboard onto an iced canvas Frame.
pub fn draw_keyboard(frame: &mut Frame, data: &MiniKeyboardData) {
    let outline_color = data.text_color;

    let half = STROKE_WIDTH / 2.0;

    for key in &data.keys {
        let x = half + (key.x * KEY_SIZE).floor();
        let y = half + key.y * KEY_SIZE;
        let w = key.w * KEY_SIZE;
        let h = key.h * KEY_SIZE;

        // Fill keys
        let fill_color = if data.unlocked.contains(&key.ch) && key.ch != ' ' {
            data.metric_colors
                .get(&key.ch)
                .cloned()
                .unwrap_or_else(|| data.primary_color)
        } else if data.pressed.contains(&key.ch) {
            data.primary_color
        } else {
            Color::TRANSPARENT
        };
        let alpha = if data.pressed.contains(&key.ch) {
            1.0
        } else {
            0.6
        };
        let fill_color = fill_color.scale_alpha(alpha);

        let fill = Path::rectangle(Point::new(x, y), Size::new(w, h));
        frame.fill(&fill, fill_color);

        let rect = Path::rectangle(Point::new(x, y), Size::new(w, h));
        frame.stroke(
            &rect,
            Stroke {
                style: outline_color.into(),
                width: STROKE_WIDTH,
                ..Stroke::default()
            },
        );
    }
}

/// A mini keyboard widget
pub struct MiniKeyboard;

impl std::fmt::Debug for MiniKeyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiniKeyboard").finish()
    }
}

impl MiniKeyboard {
    pub fn new() -> Self {
        Self
    }

    pub fn view<'a, Message: 'a>(data: MiniKeyboardData) -> crate::Element<'a, Message> {
        let w = data.pixel_width();
        let h = data.pixel_height();

        Canvas::new(MiniKeyboardProgram {
            data,
            _message: PhantomData,
        })
        .width(Length::Fixed(w))
        .height(Length::Fixed(h))
        .into()
    }
}

struct MiniKeyboardProgram<Message> {
    data: MiniKeyboardData,
    _message: PhantomData<Message>,
}

impl<Message> canvas::Program<Message, Theme> for MiniKeyboardProgram<Message> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        draw_keyboard(&mut frame, &self.data);
        vec![frame.into_geometry()]
    }
}
