use super::keyboard::Keyboard;
use super::training::{Line, Session, State};
use super::words;

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub keyboard: Keyboard,
    state: State,
    words: words::Setting,
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
    pub fn add_line(&mut self, line: Line) -> Option<words::Words> {
        self.state
            .add_line(line)
            .map(|char_set| self.words.get_words(char_set))
    }

    pub fn start_session(&self) -> Session {
        Session::new(&self.words, &self.state)
    }
}

impl Default for Profile {
    fn default() -> Self {
        let keyboard = Keyboard::default();
        let chars = keyboard.initial_chars();
        let state = State::new(chars);

        Self {
            name: "Default Profile".to_string(),
            keyboard,
            state,
            words: words::Setting::default(),
        }
    }
}
