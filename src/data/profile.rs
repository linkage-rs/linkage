use super::keyboard::Keyboard;
use super::training::{Line, Session, State};
use super::words;
use super::zipper_list::ZipperList;

#[derive(Debug, Clone)]
pub struct Profile {
    name: String,
    keyboard: Keyboard,
    state: State,
    words: words::Setting,
}

#[derive(Debug)]
pub struct Active {
    pub name: String,
    pub keyboard: Keyboard,
    pub state: State,
    pub session: Session,
}

#[derive(Debug)]
pub struct List {
    zipper: ZipperList<Profile, Active>,
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

impl Active {
    pub fn add_line(&mut self, line: Line) -> Option<words::Words> {
        self.state
            .add_line(line, &self.keyboard)
            .map(|char_set| self.session.words_setting().get_words(char_set))
    }
}

impl List {
    pub fn active(&self) -> &Active {
        self.zipper.current()
    }

    pub fn active_mut(&mut self) -> &mut Active {
        self.zipper.current_mut()
    }

    pub fn parts(self) -> (Vec<Profile>, Profile, Vec<Profile>) {
        self.zipper.into()
    }

    pub fn from_parts(parts: (Vec<Profile>, Profile, Vec<Profile>)) -> Self {
        Self {
            zipper: parts.into(),
        }
    }
}

impl From<Profile> for Active {
    fn from(profile: Profile) -> Self {
        let session = Session::new(&profile.words, &profile.state);
        Self {
            name: profile.name,
            keyboard: profile.keyboard,
            state: profile.state,
            session,
        }
    }
}

impl From<Active> for Profile {
    fn from(active: Active) -> Self {
        Self {
            name: active.name,
            keyboard: active.keyboard,
            state: active.state,
            words: active.session.words_setting().clone(),
        }
    }
}
