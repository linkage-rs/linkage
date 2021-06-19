use super::CharSet;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use serde::Deserialize;
use std::collections::HashMap;

const SCALAR: u16 = 10000;
const EN_FREQ: &[u8] = include_bytes!("../../data/en/freq.json");

#[derive(Debug, Clone)]
pub struct Random {
    letter: Vec<(u16, char)>,
    next_letter: HashMap<char, Vec<(u16, char)>>,
    dist: Uniform<u16>,
    rng: ThreadRng,
}

impl Random {
    pub fn load() -> Random {
        #[derive(Debug, Deserialize)]
        struct FreqData {
            #[serde(rename = "letter_frequency")]
            letter: HashMap<String, f32>,
            #[serde(rename = "next_letter_frequency")]
            next_letter: HashMap<String, HashMap<String, f32>>,
        }

        let freq_str = std::str::from_utf8(EN_FREQ).expect("Loading frequency data");
        let freq_data: FreqData = serde_json::from_str(freq_str).expect("Parse frequency JSON");

        let next_letter: HashMap<char, Vec<(u16, char)>> = freq_data
            .next_letter
            .into_iter()
            .map(move |(letter, next_ratio)| {
                (letter.chars().next().unwrap(), cumulative(next_ratio))
            })
            .collect();

        Random {
            letter: cumulative(freq_data.letter),
            next_letter,
            dist: Uniform::new_inclusive(0, SCALAR),
            rng: rand::thread_rng(),
        }
    }

    pub fn line(&mut self, char_set: &CharSet, min_length: usize) -> String {
        let mut line = String::new();
        while line.chars().count() < min_length {
            let word = self.word(char_set);
            line.push_str(&word);
            line.push(' ');
        }
        line
    }

    /// Generate a random word from this frequency distribution. Parameters are
    /// word length and available character set.
    pub fn word(&mut self, char_set: &CharSet) -> String {
        let mut last_letter = self.first_letter(char_set);
        let Random {
            next_letter,
            dist,
            rng,
            ..
        } = self;

        let target = (3 + dist.sample(rng) % 5) as usize;

        let mut word = String::new();
        word.push(last_letter);

        while word.len() < target {
            if let Some(freqs) = next_letter.get(&last_letter) {
                let v = dist.sample(rng);
                let test_letter = sample_cumulative(v, freqs, char_set);
                if test_letter == ' ' {
                    continue;
                }
                last_letter = test_letter;
                word.push(last_letter);
            } else {
                break;
            }
        }

        word
    }

    fn first_letter(&mut self, char_set: &CharSet) -> char {
        let Random {
            letter, dist, rng, ..
        } = self;
        loop {
            let v = dist.sample(rng);
            let l = sample_cumulative(v, letter, char_set);
            if l != ' ' {
                return l;
            }
        }
    }
}

impl Default for Random {
    fn default() -> Self {
        Random {
            letter: Vec::new(),
            next_letter: HashMap::new(),
            dist: Uniform::new_inclusive(0, SCALAR),
            rng: rand::thread_rng(),
        }
    }
}

// Takes a mapping of letter to raw frequency ratio and maps to a sorted
// vector with cumulative values between 0 and SCALAR, so sampling from a
// random distribution is easier.
fn cumulative(mapping: HashMap<String, f32>) -> Vec<(u16, char)> {
    let s32 = f32::from(SCALAR);

    let mut cumulative: Vec<(u16, char)> = mapping
        .iter()
        .map(|(letter, ratio)| ((s32 * ratio).round() as u16, letter.chars().next().unwrap()))
        .collect();
    cumulative.sort_by(|(a, _), (b, _)| b.cmp(&a));
    let mut c = 0;
    cumulative
        .into_iter()
        .map(move |(freq, letter)| {
            c += freq;
            (c.min(SCALAR), letter)
        })
        .collect()
}

fn sample_cumulative(value: u16, mapping: &[(u16, char)], char_set: &CharSet) -> char {
    if let Some(c) = mapping
        .iter()
        .filter(|(_, c)| char_set.contains(c))
        .take_while(|(v, _)| v <= &value)
        .map(|(_, c)| *c)
        .last()
    {
        c
    } else if !mapping.is_empty() {
        mapping[0].1
    } else {
        'n'
    }
}
