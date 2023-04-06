/// Iosevka Fixed Extended Thin - Weight 100
// name: "Iosevka Extended Thin",
pub const THIN: &[u8] = include_bytes!("../fonts/iosevka-fixed-extendedthin.ttf");

/// Iosevka Fixed Extended Light - Weight 300
// name: "Iosevka Extended Light",
pub const LIGHT: &[u8] = include_bytes!("../fonts/iosevka-fixed-extendedlight.ttf");

/// Iosevka Fixed Extended Medium - Weight 500
// name: "Iosevka Extended Medium",
pub const MEDIUM: &[u8] = include_bytes!("../fonts/iosevka-fixed-extendedmedium.ttf");

pub enum Font {
    Thin,
    Light,
    Medium,
}

impl Font {
    fn name(&self) -> &'static str {
        "Iosevka Fixed"
    }

    fn weight(&self) -> iced::font::Weight {
        match self {
            Font::Thin => iced::font::Weight::Thin,
            Font::Light => iced::font::Weight::Light,
            Font::Medium => iced::font::Weight::Medium,
        }
    }
}

impl From<Font> for iced::Font {
    fn from(font: Font) -> Self {
        iced::Font {
            family: iced::font::Family::Name(font.name()),
            weight: font.weight(),
            stretch: iced::font::Stretch::Expanded,
            monospaced: true,
        }
    }
}
