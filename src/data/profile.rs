use super::keyboard::Layout;
use super::training::{Line, Session, State};
use super::words;
use super::zipper_list::ZipperList;

#[derive(Debug, Clone)]
pub struct Profile {
    name: String,
    layout: Layout,
    state: State,
    words: words::Setting,
}

#[derive(Debug)]
pub struct Active {
    pub name: String,
    pub layout: Layout,
    pub state: State,
    pub session: Session,
}

#[derive(Debug)]
pub struct List {
    zipper: ZipperList<Profile, Active>,
}

#[derive(Debug, Clone, Default)]
pub struct Saved {
    prev: Vec<Profile>,
    current: Profile,
    next: Vec<Profile>,
}

impl Default for Profile {
    fn default() -> Self {
        let layout = Layout::default();
        let chars = layout.initial_chars();
        let state = State::new(chars);

        Self {
            name: "Default Profile".to_string(),
            layout,
            state,
            words: words::Setting::default(),
        }
    }
}

impl Active {
    pub fn add_line(&mut self, line: Line) -> Option<words::Words> {
        self.state
            .add_line(line, &self.layout)
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

    pub fn session(&self) -> &Session {
        &self.active().session
    }

    pub fn session_mut(&mut self) -> &mut Session {
        &mut self.active_mut().session
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

impl Default for List {
    fn default() -> Self {
        Saved::default().into()
    }
}

impl Saved {
    fn parts(self) -> (Vec<Profile>, Profile, Vec<Profile>) {
        (self.prev, self.current, self.next)
    }
}

impl From<Profile> for Active {
    fn from(profile: Profile) -> Self {
        let session = Session::new(&profile.words, &profile.state);
        Self {
            name: profile.name,
            layout: profile.layout,
            state: profile.state,
            session,
        }
    }
}

impl From<Active> for Profile {
    fn from(active: Active) -> Self {
        Self {
            name: active.name,
            layout: active.layout,
            state: active.state,
            words: active.session.words_setting().clone(),
        }
    }
}

impl From<Saved> for List {
    fn from(saved: Saved) -> Self {
        Self {
            zipper: saved.parts().into(),
        }
    }
}

impl From<List> for Saved {
    fn from(list: List) -> Self {
        let (prev, current, next) = list.zipper.into();
        Self {
            prev,
            current,
            next,
        }
    }
}
