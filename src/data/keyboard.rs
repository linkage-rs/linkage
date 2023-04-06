use super::CharSet;
use serde::{Deserialize, Serialize};

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
}
