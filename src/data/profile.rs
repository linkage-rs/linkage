use super::keyboard::Layout;
use super::training::{Line, Session, State};
use super::words;
use super::zipper_list::{Item, ZipperList};
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct Name(String);

#[derive(Debug, Clone)]
pub struct Profile {
    name: Name,
    layout: Layout,
    state: State,
    words: words::Setting,
}

#[derive(Debug)]
pub struct Active {
    pub name: Name,
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

impl Profile {
    pub fn new(name: Name, layout: Layout) -> Self {
        let chars = layout.initial_chars();
        let state = State::new(chars);

        Self {
            name,
            layout,
            state,
            words: words::Setting::default(),
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        let layout = Layout::default();
        let chars = layout.initial_chars();
        let state = State::new(chars);

        Self {
            name: Name::unchecked_from("Default Profile"),
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

    pub fn len(&self) -> usize {
        self.zipper.len()
    }

    fn iter(&self) -> impl Iterator<Item = Item<&Profile, &Active>> {
        self.zipper.iter()
    }

    pub fn select(&mut self, index: usize) -> bool {
        self.zipper.select(index)
    }

    pub fn insert_active(&mut self, profile: Profile) {
        self.zipper.push(profile);
        let len = self.zipper.len();
        self.zipper.select(len.saturating_sub(1));
    }

    /// Iterator of (name, is_active)
    pub fn names(&self) -> impl Iterator<Item = (Name, bool)> + '_ {
        self.iter().map(|item| match item {
            Item::Current(profile) => (profile.name.clone(), true),
            Item::Other(profile) => (profile.name.clone(), false),
        })
    }

    pub fn contains_name(&self, name: &Name) -> bool {
        let names: HashSet<Name> = self.names().map(|(n, _)| n).collect();
        names.contains(name)
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

impl Name {
    const MAX_LENGTH: usize = 24;

    pub fn parse(s: &str) -> Option<Name> {
        if s.trim() == s && (1..=Self::MAX_LENGTH).contains(&s.len()) {
            return Some(Self(s.to_string()));
        }

        None
    }

    pub fn new(list: &List) -> Name {
        let base: String = "New Profile".to_string();
        let existing: HashSet<String> = list.names().map(|(n, _)| String::from(n)).collect();

        if existing.contains(&base) {
            let mut counter = 2;
            loop {
                let name = format!("{} ({})", base, counter);
                if !existing.contains(&name) {
                    return Name(name);
                }
                counter += 1;
            }
        } else {
            Name(base)
        }
    }

    fn unchecked_from(s: &str) -> Name {
        Self(s.to_string())
    }
}

impl From<Name> for String {
    fn from(name: Name) -> String {
        name.0
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
