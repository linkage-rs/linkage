use super::keyboard::Keyboard;
use super::training::Event;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub profiles: Vec<Profile>,
}

#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub keyboard: Keyboard,
    events: Vec<Event>,
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
        }
    }
}
