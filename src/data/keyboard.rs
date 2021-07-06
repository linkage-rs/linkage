use super::CharSet;

#[derive(Debug, Clone)]
pub enum Layout {
    Colemak,
    ColemakDHm,
    ColemakDHmOrtholinear,
    ColemakOrtholinear,
    Dvorak,
    DvorakOrtholinear,
    Qwerty,
    QwertyOrtholinear,
    Workman,
    WorkmanOrtholinear,
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Qwerty
    }
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Layout::Colemak => "Colemak Staggered",
            Layout::ColemakDHm => "Colemak-DHm Staggered",
            Layout::ColemakDHmOrtholinear => "Colemak-DHm Ortholinear",
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
    Layout::ColemakDHm,
    Layout::Workman,
    Layout::QwertyOrtholinear,
    Layout::DvorakOrtholinear,
    Layout::ColemakOrtholinear,
    Layout::ColemakDHmOrtholinear,
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
            .filter(|&letter| !char_set.contains(&letter))
            .cloned()
            .next()
    }

    /// The order in which letters are unlocked for this layout
    pub fn letter_order(&self) -> Vec<char> {
        match self {
            Layout::Colemak => vec![
                'n', 't', 'e', 's', 'i', 'r', 'l', 'p', 'h', 'd', 'a', 'o', 'u', 'f', 'm', 'g',
                'y', 'c', 'j', 'v', 'b', 'k', 'w', 'x', 'q', 'z',
            ],
            Layout::ColemakDHm => vec![
                'n', 't', 'e', 's', 'i', 'r', 'l', 'p', 'h', 'd', 'a', 'o', 'u', 'f', 'm', 'g',
                'y', 'c', 'j', 'v', 'b', 'k', 'w', 'x', 'q', 'z',
            ],
            Layout::ColemakDHmOrtholinear => vec![
                'n', 't', 'e', 's', 'i', 'r', 'l', 'p', 'h', 'd', 'a', 'o', 'u', 'f', 'm', 'g',
                'y', 'c', 'j', 'v', 'b', 'k', 'w', 'x', 'q', 'z',
            ],
            Layout::ColemakOrtholinear => vec![
                'n', 't', 'e', 's', 'i', 'r', 'l', 'p', 'h', 'd', 'a', 'o', 'u', 'f', 'm', 'g',
                'y', 'c', 'j', 'v', 'b', 'k', 'w', 'x', 'q', 'z',
            ],
            Layout::Dvorak => vec![
                'f', 'd', 'e', 'a', 'k', 'l', 's', 'u', 'j', 'i', 'r', 'o', 't', 'm', 'c', 'y',
                'g', 'h', 'v', 'n', 'b', 'w', 'p', 'q', 'x', 'z',
            ],
            Layout::DvorakOrtholinear => vec![
                'f', 'd', 'e', 'a', 'k', 'l', 's', 'u', 'j', 'i', 'r', 'o', 't', 'm', 'c', 'y',
                'g', 'h', 'v', 'n', 'b', 'w', 'p', 'q', 'x', 'z',
            ],
            Layout::Qwerty => vec![
                'f', 'd', 'e', 'a', 'k', 'l', 's', 'u', 'j', 'i', 'r', 'o', 't', 'm', 'c', 'y',
                'g', 'h', 'v', 'n', 'b', 'w', 'p', 'q', 'x', 'z',
            ],
            Layout::QwertyOrtholinear => vec![
                'f', 'd', 'e', 'a', 'k', 'l', 's', 'u', 'j', 'i', 'r', 'o', 't', 'm', 'c', 'y',
                'g', 'h', 'v', 'n', 'b', 'w', 'p', 'q', 'x', 'z',
            ],
            Layout::Workman => vec![
                'f', 'd', 'e', 'a', 'k', 'l', 's', 'u', 'j', 'i', 'r', 'o', 't', 'm', 'c', 'y',
                'g', 'h', 'v', 'n', 'b', 'w', 'p', 'q', 'x', 'z',
            ],
            Layout::WorkmanOrtholinear => vec![
                'f', 'd', 'e', 'a', 'k', 'l', 's', 'u', 'j', 'i', 'r', 'o', 't', 'm', 'c', 'y',
                'g', 'h', 'v', 'n', 'b', 'w', 'p', 'q', 'x', 'z',
            ],
        }
    }
}
