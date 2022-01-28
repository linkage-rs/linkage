use super::dictionary::Dictionary;
use super::random::Random;
use super::CharSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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
    pub fn get_words(&self, char_set: CharSet) -> Words {
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
        char_set: CharSet,
    },
}

impl Words {
    pub fn dictionary(char_set: CharSet) -> Self {
        let dictionary = Dictionary::load();
        let char_limited = dictionary.char_limited(&char_set);
        Words::Dictionary {
            full: dictionary,
            char_limited,
        }
    }

    pub fn random(char_set: CharSet) -> Self {
        Words::Random {
            generator: Random::load(),
            char_set,
        }
    }

    pub fn line(&mut self, length: usize, least_accurate: &CharSet) -> String {
        match self {
            Words::Dictionary { char_limited, .. } => char_limited.line(length, least_accurate),
            Words::Random {
                generator,
                char_set,
            } => generator.line(char_set, length),
        }
    }
}

impl From<&Words> for Setting {
    fn from(words: &Words) -> Setting {
        match words {
            Words::Dictionary { .. } => Setting::Dictionary,
            Words::Random { .. } => Setting::Random,
        }
    }
}
