use time::{Duration, Instant, OffsetDateTime};

use std::collections::HashSet;

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
pub enum Pending {
    Unattempted(char),
    Error { target: char, error: char },
}

#[derive(Debug, Clone)]
pub struct PendingLine {
    hits: Vec<Hit>,
    pending: Vec<Pending>,
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

    pub fn add_miss(&mut self, miss: char) {
        self.misses.insert(miss);
    }

    pub fn finalize(&mut self, baseline: Instant) {
        self.dt = baseline.elapsed();
        if self.dt.whole_nanoseconds() > Self::MAX_DURATION_NS as i128 {
            self.dt = Duration::nanoseconds(Self::MAX_DURATION_NS);
        }
    }
}
