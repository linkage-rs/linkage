use super::keyboard::Layout;
use super::words::{self, Words};
use super::CharSet;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use time::{Duration, Instant, OffsetDateTime};

pub const CHARS_PER_LINE: usize = 52;
pub const NEXT_LINES: usize = 2;
pub const MAX_ERRORS: usize = 5;
pub const NUM_RECENT_TIMINGS: usize = 1000;
pub const CLEAN_ALPHA_COEFF: f32 = 1.0 / (1.0 + 50.0);
pub const MIN_CLEAN_PCT: f32 = 0.75;

#[derive(Debug, Clone)]
pub struct State {
    /// Which characters are in our set
    char_set: CharSet,
    /// How long it took to type each letter, recent sample
    timings: HashMap<char, VecDeque<Duration>>,
    /// Smoothed clean proportion
    clean: HashMap<char, f32>,
    /// Event log
    events: Vec<Event>,
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

        Self {
            char_set,
            timings: HashMap::new(),
            clean: HashMap::new(),
            events,
        }
    }

    fn char_set(&self) -> CharSet {
        self.char_set.clone()
    }

    /// Add a line of completed training. Optionally returns a new char set.
    pub fn add_line(&mut self, line: Line, layout: &Layout) -> Option<CharSet> {
        for hit in line.hits {
            if let Some(timings) = self.timings.get_mut(&hit.target) {
                timings.push_back(hit.dt);
                while timings.len() > NUM_RECENT_TIMINGS {
                    timings.pop_front();
                }
            } else {
                self.timings.insert(hit.target, vec![hit.dt].into());
            }

            let clean_signal = if hit.misses.is_empty() { 1.0 } else { 0.0 };
            if let Some(clean) = self.clean.get_mut(&hit.target) {
                *clean = clean_signal * CLEAN_ALPHA_COEFF + *clean * (1.0 - CLEAN_ALPHA_COEFF);
            } else {
                self.clean
                    .insert(hit.target, clean_signal * CLEAN_ALPHA_COEFF);
            }
        }

        let all_clean = self.clean.iter().all(|(_, &v)| v >= MIN_CLEAN_PCT);

        if all_clean {
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
    const MAX_DURATION_NS: i64 = 5_000_000_000;

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
