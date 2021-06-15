use super::training::Event;

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub name: String,
    pub layout: Layout,
}

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

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            name: "Default Keyboard".to_string(),
            layout: Layout::default(),
        }
    }
}

impl Keyboard {
    pub fn initial_events(&self) -> Vec<Event> {
        self.layout
            .letter_order()
            .iter()
            .take(6)
            .map(|&letter| Event::Unlock { letter })
            .collect()
    }
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
