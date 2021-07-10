use std::collections::HashSet;

pub mod dictionary;
pub mod keyboard;
pub mod profile;
pub mod random;
pub mod theme;
pub mod training;
pub mod words;
pub mod zipper_list;

pub use theme::Theme;
pub use words::Words;

pub type CharSet = HashSet<char>;
