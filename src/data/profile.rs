use super::keyboard::Keyboard;
use super::training::{Event, Session};
use super::Freq;

use std::collections::HashSet;

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
}

impl User {
    pub fn profile(&self) -> &Profile {
        &self.profiles[0]
    }

    pub fn profile_mut(&mut self) -> &mut Profile {
        &mut self.profiles[0]
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "Default User".to_string(),
            profiles: vec![Profile::default()],
        }
    }
}

impl Profile {
    pub fn add_events(&mut self, events: &mut Vec<Event>) {
        self.events.append(events)
    }

    pub fn start_session(&self, freq: &mut Freq) -> Session {
        let char_set = self
            .events
            .iter()
            .filter_map(|event| match event {
                Event::Unlock { letter } => Some(*letter),
                _ => None,
            })
            .collect();
        Session::from_char_set(char_set, freq)
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
