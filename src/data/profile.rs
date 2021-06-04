use super::keyboard::Keyboard;
use super::training::{Event, PendingLine};

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub keyboard: Keyboard,
    events: Vec<Event>,
    pending_lines: Vec<PendingLine>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "Default User".to_string(),
            profiles: vec![Profile::default()],
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        let keyboard = Keyboard::default();
        let events = keyboard.initial_events();

        Self {
            name: "Default Profile".to_string(),
            keyboard,
            events,
            pending_lines: Vec::new(),
        }
    }
}
