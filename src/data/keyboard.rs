use serde::{Deserialize, Serialize};

use super::CharSet;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum Layout {
    Colemak,
    ColemakDH,
    ColemakDHOrtholinear,
    ColemakOrtholinear,
    Dvorak,
    DvorakOrtholinear,
    #[default]
    Qwerty,
    QwertyOrtholinear,
    Workman,
    WorkmanOrtholinear,
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Layout::Colemak => "Colemak Staggered",
            Layout::ColemakDH => "Colemak-DH Staggered",
            Layout::ColemakDHOrtholinear => "Colemak-DH Ortholinear",
            Layout::ColemakOrtholinear => "Colemak Ortholinear",
            Layout::Dvorak => "Dvorak Staggered",
            Layout::DvorakOrtholinear => "Dvorak Ortholinear",
            Layout::Qwerty => "Qwerty Staggered",
            Layout::QwertyOrtholinear => "Qwerty Ortholinear",
            Layout::Workman => "Workman Staggered",
            Layout::WorkmanOrtholinear => "Workman Ortholinear",
        };

        write!(f, "{}", s)
    }
}

pub const ALL: &[Layout] = &[
    Layout::Qwerty,
    Layout::Dvorak,
    Layout::Colemak,
    Layout::ColemakDH,
    Layout::Workman,
    Layout::QwertyOrtholinear,
    Layout::DvorakOrtholinear,
    Layout::ColemakOrtholinear,
    Layout::ColemakDHOrtholinear,
    Layout::WorkmanOrtholinear,
];

impl Layout {
    pub fn initial_chars(&self) -> Vec<char> {
        self.letter_order()[0..6].to_vec()
    }

    /// Get the next character in the list, given the current character set
    pub fn next_char(&self, char_set: &CharSet) -> Option<char> {
        self.letter_order()
            .iter()
            .filter(|&letter| !char_set.contains(letter))
            .cloned()
            .next()
    }

    /// The order in which letters are unlocked for this layout
    pub fn letter_order(&self) -> Vec<char> {
        match self {
            Layout::Colemak => vec![
                'n', 't', 'e', 's', 'i', 'r', 'o', 'a', 'l', 'p', 'u', 'f', 'y', 'w', 'm', 'v',
                'c', 'h', 'd', 'j', 'g', 'k', 'b', 'x', 'q', 'z',
            ],
            Layout::ColemakDH => vec![
                'n', 't', 'e', 's', 'i', 'r', 'o', 'a', 'l', 'p', 'u', 'f', 'y', 'w', 'h', 'd',
                'c', 'm', 'g', 'j', 'b', 'k', 'v', 'x', 'q', 'z',
            ],
            Layout::ColemakDHOrtholinear => vec![
                'n', 't', 'e', 's', 'i', 'r', 'o', 'a', 'l', 'p', 'u', 'f', 'y', 'w', 'h', 'd',
                'c', 'm', 'g', 'j', 'b', 'k', 'v', 'x', 'q', 'z',
            ],
            Layout::ColemakOrtholinear => vec![
                'n', 't', 'e', 's', 'i', 'r', 'o', 'a', 'l', 'p', 'u', 'f', 'y', 'w', 'm', 'v',
                'c', 'h', 'd', 'j', 'g', 'k', 'b', 'x', 'q', 'z',
            ],
            Layout::Dvorak => vec![
                'h', 'u', 't', 'e', 'n', 'o', 'g', 'p', 's', 'a', 'c', 'd', 'i', 'r', 'l', 'm',
                'k', 'f', 'y', 'w', 'j', 'b', 'v', 'q', 'x', 'z',
            ],
            Layout::DvorakOrtholinear => vec![
                'h', 'u', 't', 'e', 'n', 'o', 'g', 'p', 's', 'a', 'c', 'd', 'i', 'r', 'l', 'm',
                'k', 'f', 'y', 'w', 'j', 'b', 'v', 'q', 'x', 'z',
            ],
            Layout::Qwerty => vec![
                'a', 's', 'd', 'f', 'e', 'j', 'k', 'i', 'l', 'u', 'r', 'o', 'w', 'm', 'v', 'c',
                'h', 'g', 'y', 't', 'n', 'b', 'p', 'x', 'q', 'z',
            ],
            Layout::QwertyOrtholinear => vec![
                'a', 's', 'd', 'f', 'e', 'j', 'k', 'i', 'l', 'u', 'r', 'o', 'w', 'm', 'v', 'c',
                'h', 'g', 'y', 't', 'n', 'b', 'p', 'x', 'q', 'z',
            ],
            Layout::Workman => vec![
                'n', 't', 'e', 'h', 'o', 's', 'i', 'a', 'f', 'w', 'u', 'r', 'p', 'd', 'l', 'c',
                'm', 'y', 'g', 'j', 'b', 'k', 'v', 'x', 'q', 'z',
            ],
            Layout::WorkmanOrtholinear => vec![
                'n', 't', 'e', 'h', 'o', 's', 'i', 'a', 'f', 'w', 'u', 'r', 'p', 'd', 'l', 'c',
                'm', 'y', 'g', 'j', 'b', 'k', 'v', 'x', 'q', 'z',
            ],
        }
    }

    pub fn is_ortholinear(&self) -> bool {
        match self {
            Layout::Colemak
            | Layout::ColemakDH
            | Layout::Dvorak
            | Layout::Qwerty
            | Layout::Workman => false,
            Layout::ColemakDHOrtholinear
            | Layout::ColemakOrtholinear
            | Layout::DvorakOrtholinear
            | Layout::QwertyOrtholinear
            | Layout::WorkmanOrtholinear => true,
        }
    }

    /// Returns the rows of characters on this keyboard layout.
    pub fn rows(&self) -> Vec<Vec<char>> {
        match self {
            Layout::Qwerty => vec![
                vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']'],
                vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\''],
                vec!['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
            ],
            Layout::QwertyOrtholinear => vec![
                vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
                vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
                vec!['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
            ],
            Layout::Dvorak => vec![
                vec!['\'', ',', '.', 'p', 'y', 'f', 'g', 'c', 'r', 'l', '/', '='],
                vec!['a', 'o', 'e', 'u', 'i', 'd', 'h', 't', 'n', 's', '-'],
                vec![';', 'q', 'j', 'k', 'x', 'b', 'm', 'w', 'v', 'z'],
            ],
            Layout::DvorakOrtholinear => vec![
                vec!['\'', ',', '.', 'p', 'y', 'f', 'g', 'c', 'r', 'l'],
                vec!['a', 'o', 'e', 'u', 'i', 'd', 'h', 't', 'n', 's'],
                vec![';', 'q', 'j', 'k', 'x', 'b', 'm', 'w', 'v', 'z'],
            ],
            Layout::Colemak => vec![
                vec!['q', 'w', 'f', 'p', 'g', 'j', 'l', 'u', 'y', ';', '[', ']'],
                vec!['a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o', '\''],
                vec!['z', 'x', 'c', 'v', 'b', 'k', 'm', ',', '.', '/'],
            ],
            Layout::ColemakOrtholinear => vec![
                vec!['q', 'w', 'f', 'p', 'g', 'j', 'l', 'u', 'y', ';'],
                vec!['a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o'],
                vec!['z', 'x', 'c', 'v', 'b', 'k', 'm', ',', '.', '/'],
            ],
            Layout::ColemakDH => vec![
                vec!['q', 'w', 'f', 'p', 'b', 'j', 'l', 'u', 'y', ';', '[', ']'],
                vec!['a', 'r', 's', 't', 'g', 'm', 'n', 'e', 'i', 'o', '\''],
                vec!['z', 'x', 'c', 'd', 'v', 'k', 'h', ',', '.', '/'],
            ],
            Layout::ColemakDHOrtholinear => vec![
                vec!['q', 'w', 'f', 'p', 'b', 'j', 'l', 'u', 'y', ';'],
                vec!['a', 'r', 's', 't', 'g', 'm', 'n', 'e', 'i', 'o'],
                vec!['z', 'x', 'c', 'd', 'v', 'k', 'h', ',', '.', '/'],
            ],
            Layout::Workman => vec![
                vec!['q', 'd', 'r', 'w', 'b', 'j', 'f', 'u', 'p', ';', '[', ']'],
                vec!['a', 's', 'h', 't', 'g', 'y', 'n', 'e', 'o', 'i', '\''],
                vec!['z', 'x', 'm', 'c', 'v', 'k', 'l', ',', '.', '/'],
            ],
            Layout::WorkmanOrtholinear => vec![
                vec!['q', 'd', 'r', 'w', 'b', 'j', 'f', 'u', 'p', ';'],
                vec!['a', 's', 'h', 't', 'g', 'y', 'n', 'e', 'o', 'i'],
                vec!['z', 'x', 'm', 'c', 'v', 'k', 'l', ',', '.', '/'],
            ],
        }
    }

    /// Get the physical key layout for rendering the mini keyboard.
    /// Returns a list of PhysicalKey entries with positions in key-unit coordinates.
    /// The space bar is included as ' '.
    pub fn physical_keys(&self) -> Vec<PhysicalKey> {
        let rows = self.rows();
        if self.is_ortholinear() {
            self.ortholinear_physical_keys(&rows)
        } else {
            self.staggered_physical_keys(&rows)
        }
    }

    /// Total width and height of the keyboard in key units.
    pub fn keyboard_size(&self) -> (f32, f32) {
        if self.is_ortholinear() {
            // 10 columns wide, 3 letter rows + 1 space bar row = 4
            (10.0, 4.0)
        } else {
            (12.0, 4.0)
        }
    }

    fn ortholinear_physical_keys(&self, rows: &[Vec<char>]) -> Vec<PhysicalKey> {
        let mut keys = Vec::new();

        // Place each letter row left-aligned at column 0 in a 10-wide grid.
        for (row_idx, row) in rows.iter().enumerate() {
            let y = row_idx as f32;
            for (col_idx, &ch) in row.iter().enumerate() {
                keys.push(PhysicalKey {
                    ch,
                    x: col_idx as f32,
                    y,
                    w: 1.0,
                    h: 1.0,
                });
            }
        }

        // Space bar: 2 keys wide, centered within the 10-column grid,
        // on the row after the last letter row.
        let space_row = rows.len() as f32;
        keys.push(PhysicalKey {
            ch: ' ',
            x: 4.0,
            y: space_row,
            w: 2.0,
            h: 1.0,
        });

        keys
    }

    fn staggered_physical_keys(&self, rows: &[Vec<char>]) -> Vec<PhysicalKey> {
        let mut keys = Vec::new();

        let stagger_offsets: [f32; 3] = [0.0, 0.5, 1.0];

        for (row_idx, row) in rows.iter().enumerate() {
            let y = row_idx as f32;
            let x_offset = stagger_offsets.get(row_idx).copied().unwrap_or(0.0);
            for (col_idx, &ch) in row.iter().enumerate() {
                keys.push(PhysicalKey {
                    ch,
                    x: x_offset + col_idx as f32,
                    y,
                    w: 1.0,
                    h: 1.0,
                });
            }
        }

        // Space bar: 5 keys wide, centered in the keyboard width
        keys.push(PhysicalKey {
            ch: ' ',
            x: 3.5,
            y: 3.0,
            w: 5.0,
            h: 1.0,
        });

        keys
    }
}

/// Which metric to use for the mini keyboard fill color.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum KeyboardMetric {
    #[default]
    Accuracy,
    Speed,
}

impl std::fmt::Display for KeyboardMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardMetric::Accuracy => write!(f, "Accuracy"),
            KeyboardMetric::Speed => write!(f, "Speed"),
        }
    }
}

pub const ALL_KEYBOARD_METRICS: &[KeyboardMetric] =
    &[KeyboardMetric::Accuracy, KeyboardMetric::Speed];

/// User settings for the mini keyboard overlay on the training screen.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MiniKeyboardSettings {
    pub show: bool,
    pub metric: KeyboardMetric,
}

impl Default for MiniKeyboardSettings {
    fn default() -> Self {
        Self {
            show: true,
            metric: KeyboardMetric::default(),
        }
    }
}

/// A key on the physical keyboard, with its position and width (in key units).
#[derive(Debug, Clone, Copy)]
pub struct PhysicalKey {
    /// The lowercase character this key produces
    pub ch: char,
    /// X offset in key-width units from the left edge of the keyboard
    pub x: f32,
    /// Y offset in key-height units from the top of the keyboard
    pub y: f32,
    /// Width of the key in key-width units
    pub w: f32,
    /// Height of the key in key-height units
    pub h: f32,
}
