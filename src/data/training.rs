use super::keyboard::Layout;
use super::words::{self, Words};
use super::CharSet;
use itertools::Itertools;
use statrs::statistics::{self, Distribution, Statistics};
use std::collections::{HashMap, HashSet, VecDeque};
use time::{Duration, Instant, OffsetDateTime};

pub const CHARS_PER_LINE: usize = 52;
pub const NEXT_LINES: usize = 1;
pub const MAX_ERRORS: usize = 5;
pub const NUM_RECENT_TIMINGS: usize = 16;
pub const CLEAN_ALPHA_COEFF: f32 = 1.0 / (1.0 + 16.0);
pub const MIN_CLEAN_PCT: f32 = 0.75;
pub const MIN_WPM: f64 = 30.0;
const CHARACTERS_PER_WORD: f64 = 5.0;

#[derive(Debug, Clone)]
pub struct State {
    /// Which characters are in our set
    char_set: CharSet,
    /// Stats for each letter
    pub timings: HashMap<char, Stats>,
    /// Smoothed clean proportion
    clean: HashMap<char, f32>,
    /// Event log
    events: Vec<Event>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct WordsPerMinute(f64);

#[derive(Debug, Clone)]
pub struct Stats {
    raw: Vec<WordsPerMinute>,
    pub wpm_mean: WordsPerMinute,
    pub wpm_harmonic_mean: WordsPerMinute,
}

/// Event log messages that record when certain state transitions occurred.
#[derive(Debug, Clone)]
pub enum Event {
    /// New letter added to our training set
    Unlock { letter: char, time: OffsetDateTime },
    // /// Computed progress point
    // Progress {
    //     time: OffsetDateTime,
    //     total_time_training: Duration,
    //     total_lines_typed: u64,
    //     total_characters_typed: u64,
    //     top_speed_wpm: f64,
    //     average_speed_wpm: f64,
    //     num_characters: u8,
    // },
    // /// Statistical rollup
    // Checkpoint {},
}

#[derive(Debug, Clone)]
pub struct Session {
    /// Words to choose from
    words: Words,
    /// The instant when the last hit was completed
    baseline: Instant,
    /// The current hit we're working on
    pub active_hit: Hit,
    /// List of completed hits
    pub hits: Vec<Hit>,
    /// The target characters we have yet to hit
    pub targets: VecDeque<char>,
    // The current errors, which must be cleared before moving forward
    pub errors: Vec<char>,
    /// The next few lines of target text
    pub next_lines: Vec<String>,
}

/// A line of training completed
#[derive(Debug, Clone)]
pub struct Line {
    hits: Vec<Hit>,
    time: OffsetDateTime,
}

/// A successful keystroke
#[derive(Debug, Clone)]
pub struct Hit {
    /// The character to type
    target: char,
    /// The previous character typed
    prev: char,
    /// Incorrect keys hit instead of the target
    misses: CharSet,
    /// Time required to hit the target. Limited to some maximum threshold so
    /// we can leave and come back to training without blowing up any averages.
    dt: Duration,
}

impl State {
    pub fn new(chars: Vec<char>) -> Self {
        let char_set = chars.iter().cloned().collect();
        let events = chars.iter().map(|&letter| Event::unlock(letter)).collect();
        let clean = chars.iter().map(|&letter| (letter, 0.0)).collect();
        let timings = chars
            .iter()
            .map(|&letter| (letter, Stats::default()))
            .collect();

        Self {
            char_set,
            timings,
            clean,
            events,
        }
    }

    fn char_set(&self) -> CharSet {
        self.char_set.clone()
    }

    /// Add a line of completed training. Optionally returns a new char set.
    pub fn add_line(&mut self, line: Line, layout: &Layout) -> Option<CharSet> {
        for hit in line.hits.iter().skip(1) {
            if hit.misses.is_empty() {
                self.timings.entry(hit.target).or_default().push(hit.dt);
            }

            let clean_signal = if hit.misses.is_empty() { 1.0 } else { 0.0 };
            if let Some(clean) = self.clean.get_mut(&hit.target) {
                *clean = clean_signal * CLEAN_ALPHA_COEFF + *clean * (1.0 - CLEAN_ALPHA_COEFF);
            } else {
                self.clean
                    .insert(hit.target, clean_signal * CLEAN_ALPHA_COEFF);
            }
        }
        self.timings
            .iter_mut()
            .for_each(|(_, stats)| stats.recompute());

        let all_clean = self.clean.iter().all(|(_, &v)| v >= MIN_CLEAN_PCT);
        let all_fast_enough = self
            .timings
            .iter()
            .all(|(_, stats)| f64::from(stats.wpm_harmonic_mean) >= MIN_WPM);

        if all_clean && all_fast_enough {
            if let Some(letter) = layout.next_char(&self.char_set) {
                self.char_set.insert(letter);
                self.clean.insert(letter, 0.0);
                self.events.push(Event::unlock(letter));

                return Some(self.char_set.clone());
            }
        }

        None
    }

    pub fn clean_letters(&self) -> Vec<(char, f32)> {
        use std::cmp::Ordering;
        self.clean
            .clone()
            .into_iter()
            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal))
            .collect()
    }
}

impl From<Duration> for WordsPerMinute {
    fn from(duration: Duration) -> Self {
        let seconds_per_character = duration.as_seconds_f64();
        let characters_per_minute = 60.0 / seconds_per_character;
        Self(characters_per_minute / CHARACTERS_PER_WORD)
    }
}

impl From<f64> for WordsPerMinute {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

impl From<WordsPerMinute> for f64 {
    fn from(wpm: WordsPerMinute) -> f64 {
        wpm.0
    }
}

impl Stats {
    pub fn push(&mut self, duration: Duration) {
        self.raw.push(duration.into());
        while self.raw.len() > NUM_RECENT_TIMINGS {
            self.raw.remove(0);
        }
    }

    pub fn recompute(&mut self) {
        let mut raw: Vec<f64> = self.raw.iter().map(move |&v| f64::from(v)).collect();
        let data = statistics::Data::new(raw.as_mut_slice());
        self.wpm_mean = data.mean().unwrap_or_default().into();
        self.wpm_harmonic_mean = raw.harmonic_mean().into();
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            raw: Vec::new(),
            wpm_mean: WordsPerMinute::default(),
            wpm_harmonic_mean: WordsPerMinute::default(),
        }
    }
}

impl Event {
    fn unlock(letter: char) -> Self {
        Self::Unlock {
            letter,
            time: OffsetDateTime::now_utc(),
        }
    }
}

impl Session {
    pub fn new(setting: &words::Setting, state: &State) -> Self {
        let mut words = setting.get_words(state.char_set());
        let line = words.line(CHARS_PER_LINE);

        let mut targets: VecDeque<char> = line.chars().collect::<Vec<char>>().into();
        let first_letter = targets.pop_front().unwrap_or(' ');

        let mut next_lines = Vec::new();
        while next_lines.len() < NEXT_LINES {
            next_lines.push(words.line(CHARS_PER_LINE));
        }

        Self {
            words,
            baseline: Instant::now(),
            active_hit: Hit::new(first_letter, ' '),
            targets,
            hits: Vec::new(),
            errors: Vec::new(),
            next_lines,
        }
    }

    pub fn apply_char(&mut self, c: char) -> Option<Line> {
        if self.errors.is_empty() && c == self.active_hit.target {
            self.active_hit.finalize(self.baseline);
            self.hits.push(self.active_hit.clone());
            self.baseline = Instant::now();

            if let Some(next_target) = self.targets.pop_front() {
                self.active_hit = self.active_hit.next(next_target);
            } else {
                let line = Line {
                    hits: self.hits.clone(),
                    time: OffsetDateTime::now_utc(),
                };
                self.hits.clear();

                return Some(line);
            }
        } else {
            self.active_hit.add_miss(c);
            if self.errors.len() == MAX_ERRORS {
                self.errors.pop();
            }
            self.errors.push(c);
        }

        None
    }

    pub fn fill_next_lines(&mut self) {
        while self.next_lines.len() < NEXT_LINES + 1 {
            // TODO: Weighted character set selection:
            // - More words with characters that are our lower hit percentage
            // - Characters that are our slowest
            self.next_lines.push(self.words.line(CHARS_PER_LINE));
        }
        for c in self.next_lines.remove(0).chars() {
            self.targets.push_back(c);
        }

        let next_target = self.targets.pop_front().unwrap_or(' ');
        self.active_hit = self.active_hit.next(next_target);
    }

    pub fn backspace(&mut self) {
        if !self.errors.is_empty() {
            self.errors.pop();
        }
    }

    pub fn update_words(&mut self, words: Words) {
        self.words = words;
    }

    pub fn words_setting(&self) -> words::Setting {
        (&self.words).into()
    }
}

impl Hit {
    pub const MAX_DURATION_NS: i64 = 5_000_000_000;

    pub fn new(target: char, prev: char) -> Self {
        Self {
            target,
            prev,
            misses: HashSet::with_capacity(4),
            dt: Duration::zero(),
        }
    }

    pub fn next(&self, target: char) -> Self {
        Self::new(target, self.target)
    }

    pub fn add_miss(&mut self, miss: char) {
        self.misses.insert(miss);
    }

    pub fn finalize(&mut self, baseline: Instant) {
        self.dt = baseline.elapsed();
        if self.dt.whole_nanoseconds() > Self::MAX_DURATION_NS as i128 {
            self.dt = Duration::nanoseconds(Self::MAX_DURATION_NS);
        }
    }

    pub fn target(&self) -> char {
        self.target
    }

    pub fn is_dirty(&self) -> bool {
        !self.misses.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct TriplePoint {
    lower: f32,
    mid: f32,
    upper: f32,
}

impl TriplePoint {
    pub fn new(lower: f32, mid: f32, upper: f32) -> Option<Self> {
        (lower < mid && mid < upper).then(|| Self { lower, mid, upper })
    }

    /// Map values on two linear scales between [lower, mid] and [mid, upper]
    /// Values outside lower and upper are clamped to 0.0 and 1.0
    pub fn value(&self, val: f32) -> f32 {
        if val <= self.lower {
            0.0
        } else if val <= self.mid {
            0.5 * (val - self.lower) / (self.mid - self.lower)
        } else if val <= self.upper {
            0.5 + 0.5 * (val - self.mid) / (self.upper - self.mid)
        } else {
            1.0
        }
    }
}
