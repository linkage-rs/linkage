use super::CharSet;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

const EN_WORDS: &[u8] = include_bytes!("../../data/en/popular.txt");

#[derive(Debug, Clone)]
pub struct Dictionary {
    words: Vec<String>,
    dist: Uniform<usize>,
    rng: ThreadRng,
    unit_dist: Uniform<f32>,
}

impl Dictionary {
    pub fn load() -> Dictionary {
        let raw = std::str::from_utf8(EN_WORDS).expect("Loading raw words");
        let words: Vec<String> = raw
            .split('\n')
            .filter_map(|s| (!s.is_empty()).then(|| s.trim().to_string()))
            .collect();
        let dist = Uniform::new(0, words.len());
        let unit_dist = Uniform::new_inclusive(0.0, 1.0);

        Dictionary {
            words,
            dist,
            rng: rand::thread_rng(),
            unit_dist,
        }
    }

    /// Get subset of the wordlist limited to words containing only the
    /// provided characters
    pub fn char_limited(&self, char_set: &CharSet) -> Self {
        let words: Vec<String> = self
            .words
            .iter()
            .filter(|word| word.chars().all(|c| char_set.contains(&c)))
            .cloned()
            .collect();
        let dist = Uniform::new(0, words.len());

        Dictionary {
            words,
            dist,
            rng: rand::thread_rng(),
            unit_dist: self.unit_dist.clone(),
        }
    }

    /// Get subset of the wordlist limited to words of a specific length
    pub fn length_limited_exact(&self, length: usize) -> Option<Self> {
        let words: Vec<String> = self
            .words
            .iter()
            .filter(|word| word.chars().count() == length)
            .cloned()
            .collect();
        (!words.is_empty()).then(|| {
            let dist = Uniform::new(0, words.len());

            Dictionary {
                words,
                dist,
                rng: rand::thread_rng(),
                unit_dist: self.unit_dist.clone(),
            }
        })
    }

    /// Get subset of the wordlist limited to words of a maxiimum length
    pub fn length_limited_max(&self, max_length: usize) -> Option<Self> {
        let words: Vec<String> = self
            .words
            .iter()
            .filter(|word| word.chars().count() <= max_length)
            .cloned()
            .collect();
        (!words.is_empty()).then(|| {
            let dist = Uniform::new(0, words.len());

            Dictionary {
                words,
                dist,
                rng: rand::thread_rng(),
                unit_dist: self.unit_dist.clone(),
            }
        })
    }

    /// Random line of words at most `length` characters long
    pub fn line(&mut self, length: usize, least_accurate: &CharSet) -> String {
        let mut best = String::new();
        let mut metric = i32::MAX;

        for _ in 0..100 {
            let mut line = String::new();
            let mut line_len = 0;
            while line_len <= length {
                let word = self.sample();
                let word_chars = word.chars().collect::<CharSet>();

                // With a random probability, discard the word if it contains
                // none of the lowest accuracy letters.
                let contains_least_accurate =
                    least_accurate.intersection(&word_chars).any(|_| true);
                if !contains_least_accurate && self.unit_dist.sample(&mut self.rng) < 0.5 {
                    continue;
                }

                line.push_str(&word);
                line.push(' ');

                line_len = line.chars().count();
                if line_len == length {
                    return line;
                }

                let line_metric = length as i32 - line_len as i32;
                if line_metric >= 0 && line_metric < metric {
                    metric = line_metric;
                    best = line.clone();
                }
            }
        }
        best
    }

    fn sample(&mut self) -> String {
        if self.words.is_empty() {
            return "error".to_string();
        }
        let index = self.dist.sample(&mut self.rng).min(self.words.len() - 1);
        self.words[index].clone()
    }
}
