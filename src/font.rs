use iced::Font;

/// Iosevka Fixed Extended Thin - Weight 100
pub const THIN: Font = Font::External {
    name: "Iosevka Extended Thin",
    bytes: include_bytes!("../fonts/iosevka-fixed-extendedthin.ttf"),
};

/// Iosevka Fixed Extended Light - Weight 300
pub const LIGHT: Font = Font::External {
    name: "Iosevka Extended Light",
    bytes: include_bytes!("../fonts/iosevka-fixed-extendedlight.ttf"),
};

/// Iosevka Fixed Extended Medium - Weight 500
pub const MEDIUM: Font = Font::External {
    name: "Iosevka Extended Medium",
    bytes: include_bytes!("../fonts/iosevka-fixed-extendedmedium.ttf"),
};
