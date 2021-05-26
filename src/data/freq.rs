use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use serde::Deserialize;

use std::collections::HashMap;

const SCALAR: u16 = 10000;
const EN_FREQ: &[u8] = include_bytes!("../../data/en/freq.json");

#[derive(Debug)]
pub struct Freq {
    letter: Vec<(u16, char)>,
    next_letter: HashMap<char, Vec<(u16, char)>>,
    dist: Uniform<u16>,
    rng: ThreadRng,
}

impl Freq {
    pub fn load() -> Option<Freq> {
        #[derive(Debug, Deserialize)]
        struct FreqData {
            #[serde(rename = "letter_frequency")]
            letter: HashMap<String, f32>,
            #[serde(rename = "next_letter_frequency")]
            next_letter: HashMap<String, HashMap<String, f32>>,
        }

        let freq_str = std::str::from_utf8(EN_FREQ).ok()?;
        let freq_data: FreqData = serde_json::from_str(freq_str).ok()?;

        let next_letter: HashMap<char, Vec<(u16, char)>> = freq_data
            .next_letter
            .into_iter()
            .map(move |(letter, next_ratio)| {
                (letter.chars().next().unwrap(), cumulative(next_ratio))
            })
            .collect();

        let freq = Freq {
            letter: cumulative(freq_data.letter),
            next_letter,
            dist: Uniform::new_inclusive(0, SCALAR),
            rng: rand::thread_rng(),
        };

        Some(freq)
    }

    /// Generate a random word from this frequency distribution. Parameters are
    /// word length and available character set.
    pub fn random_word(&mut self) -> String {
        let mut last_letter = self.random_first_letter();
        let Freq {
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
                let test_letter = sample_cumulative(v, freqs);
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

    fn random_first_letter(&mut self) -> char {
        let Freq {
            letter, dist, rng, ..
        } = self;
        loop {
            let v = dist.sample(rng);
            let l = sample_cumulative(v, letter);
            if l != ' ' {
                return l;
            }
        }
    }
}

impl Default for Freq {
    fn default() -> Self {
        Freq {
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

fn sample_cumulative(value: u16, mapping: &[(u16, char)]) -> char {
    if let Some((_, c)) = mapping.iter().take_while(|(v, _)| v <= &value).last() {
        *c
    } else if !mapping.is_empty() {
        mapping[0].1
    } else {
        'n'
    }
}
