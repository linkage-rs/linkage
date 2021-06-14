use super::Freq;
use std::collections::{HashSet, VecDeque};
use time::{Duration, Instant, OffsetDateTime};

pub const CHARS_PER_LINE: usize = 52;
pub const NEXT_LINES: usize = 2;
pub const MAX_ERRORS: usize = 5;

/// Event log messages. Apply all messages in order to return to the current
/// training state. Hit events are rolled up into Checkpoint messages if we
/// need to compact the log.
#[derive(Debug, Clone)]
pub enum Event {
    /// New letter added to our training set
    Unlock { letter: char },
    /// A line of training completed
    Line {
        hits: Vec<Hit>,
        time: OffsetDateTime,
    },
    /// Computed progress point
    Progress {
        time: OffsetDateTime,
        total_time_training: Duration,
        total_characters_typed: u64,
        top_speed_wpm: f64,
        average_speed_wpm: f64,
        num_characters: u8,
    },
    /// Hit statistical rollup. Purging Line events before a Checkpoint is safe.
    Checkpoint {},
}

#[derive(Debug, Clone)]
pub struct Session {
    /// The character choices
    char_set: HashSet<char>,
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

/// A successful keystroke
#[derive(Debug, Clone)]
pub struct Hit {
    /// The character to type
    target: char,
    /// The previous character typed
    prev: char,
    /// Incorrect keys hit instead of the target
    misses: HashSet<char>,
    /// Time required to hit the target. Limited to some maximum threshold so
    /// we can leave and come back to training without blowing up any averages.
    dt: Duration,
}

impl Session {
    // TODO: Actually use a character set..
    pub fn from_char_set(char_set: HashSet<char>, freq: &mut Freq) -> Self {
        let line = freq.random_line(&char_set, CHARS_PER_LINE);

        let mut targets: VecDeque<char> = line.chars().collect::<Vec<char>>().into();
        let first_letter = targets.pop_front().unwrap_or(' ');

        let mut next_lines = Vec::new();
        while next_lines.len() < NEXT_LINES {
            next_lines.push(freq.random_line(&char_set, CHARS_PER_LINE));
        }

        Self {
            char_set,
            baseline: Instant::now(),
            active_hit: Hit::new(first_letter, ' '),
            targets,
            hits: Vec::new(),
            errors: Vec::new(),
            next_lines,
        }
    }

    pub fn apply_char(&mut self, c: char, freq: &mut Freq) -> Option<Vec<Event>> {
        if self.errors.is_empty() && c == self.active_hit.target {
            self.active_hit.finalize(self.baseline);
            self.hits.push(self.active_hit.clone());
            self.baseline = Instant::now();

            if let Some(next_target) = self.targets.pop_front() {
                self.active_hit = self.active_hit.next(next_target);
            } else {
                while self.next_lines.len() < NEXT_LINES + 1 {
                    // TODO: Actually use a character set for the line..
                    self.next_lines
                        .push(freq.random_line(&self.char_set, CHARS_PER_LINE));
                }
                for c in self.next_lines.remove(0).chars() {
                    self.targets.push_back(c);
                }

                let next_target = self.targets.pop_front().unwrap_or(' ');
                self.active_hit = self.active_hit.next(next_target);

                let event = Event::Line {
                    hits: self.hits.clone(),
                    time: OffsetDateTime::now_utc(),
                };
                self.hits.clear();

                // TODO: Check if we need to unlock new characters or generate
                // a Progress event

                return Some(vec![event]);
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
