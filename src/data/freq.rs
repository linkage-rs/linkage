use serde::Deserialize;

use std::collections::{BTreeMap, HashMap};

const SCALAR: u16 = 10000;
const EN_FREQ: &[u8] = include_bytes!("../../data/en/freq.json");

#[derive(Debug, Default)]
pub struct Freq {
    letter: BTreeMap<u16, char>,
    next_letter: HashMap<char, BTreeMap<u16, char>>,
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

        let s32 = f32::from(SCALAR);

        let letter: Vec<(u16, char)> = freq_data
            .letter
            .iter()
            .map(|(letter, ratio)| ((s32 * ratio).round() as u16, letter.chars().next().unwrap()))
            .collect();

        let next_letter: HashMap<char, BTreeMap<u16, char>> = freq_data
            .next_letter
            .iter()
            .map(|(letter, next_ratio)| {
                let c = letter.chars().next().unwrap();

                let next_map: BTreeMap<u16, char> = next_ratio
                    .iter()
                    .map(|(next_letter, ratio)| {
                        (
                            (s32 * ratio).round() as u16,
                            next_letter.chars().next().unwrap(),
                        )
                    })
                    .collect();

                (c, next_map)
            })
            .collect();

        let freq = Freq {
            letter: letter.into_iter().collect(),
            next_letter,
        };

        Some(freq)
    }
}
