use super::dictionary::Dictionary;
use super::random::Random;

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Setting {
    Dictionary,
    Random,
}

impl Default for Setting {
    fn default() -> Self {
        Self::Dictionary
    }
}

impl Setting {
    pub fn get_words(&self, char_set: HashSet<char>) -> Words {
        match self {
            Setting::Dictionary => Words::dictionary(char_set),
            Setting::Random => Words::random(char_set),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Words {
    Dictionary {
        full: Dictionary,
        char_limited: Dictionary,
    },
    Random {
        generator: Random,
        char_set: HashSet<char>,
    },
}

impl Words {
    pub fn dictionary(char_set: HashSet<char>) -> Self {
        let dictionary = Dictionary::load();
        let char_limited = dictionary.char_limited(&char_set);
        Words::Dictionary {
            full: dictionary,
            char_limited,
        }
    }

    pub fn random(char_set: HashSet<char>) -> Self {
        Words::Random {
            generator: Random::load(),
            char_set,
        }
    }

    pub fn line(&mut self, length: usize) -> String {
        match self {
            Words::Dictionary { char_limited, .. } => char_limited.line(length),
            Words::Random {
                generator,
                char_set,
            } => generator.line(char_set, length),
        }
    }
}
