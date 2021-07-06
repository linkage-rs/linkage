use super::profile::{self, Profile};
use super::training::Session;
use super::zipper_list::ZipperList;

#[derive(Debug, Clone)]
pub struct User {
    name: String,
    profiles: (Vec<Profile>, Profile, Vec<Profile>),
}

#[derive(Debug)]
pub struct Active {
    pub name: String,
    pub profiles: profile::List,
}

#[derive(Debug)]
pub struct List {
    zipper: ZipperList<User, Active>,
}

#[derive(Debug, Clone, Default)]
pub struct Saved {
    prev: Vec<User>,
    current: User,
    next: Vec<User>,
}

impl Saved {
    fn parts(self) -> (Vec<User>, User, Vec<User>) {
        (self.prev, self.current, self.next)
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "Default User".to_string(),
            profiles: (Vec::new(), Profile::default(), Vec::new()),
        }
    }
}

impl List {
    pub fn active(&self) -> &Active {
        self.zipper.current()
    }

    pub fn active_mut(&mut self) -> &mut Active {
        self.zipper.current_mut()
    }

    pub fn profile(&self) -> &profile::Active {
        self.active().profiles.active()
    }

    pub fn profile_mut(&mut self) -> &mut profile::Active {
        self.active_mut().profiles.active_mut()
    }

    pub fn session(&self) -> &Session {
        &self.profile().session
    }

    pub fn session_mut(&mut self) -> &mut Session {
        &mut self.profile_mut().session
    }
}

impl Default for List {
    fn default() -> Self {
        Saved::default().into()
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

impl From<User> for Active {
    fn from(user: User) -> Self {
        Self {
            name: user.name,
            profiles: profile::List::from_parts(user.profiles),
        }
    }
}

impl From<Active> for User {
    fn from(active: Active) -> Self {
        Self {
            name: active.name,
            profiles: active.profiles.parts(),
        }
    }
}
