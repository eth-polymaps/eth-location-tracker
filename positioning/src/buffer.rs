use crate::signal::Signal;
use chrono::{Duration, Utc};
use std::collections::VecDeque;

pub struct Buffer {
    signals: VecDeque<Signal>, // VecDeque to store the signals
    max_size: usize,
}
impl Buffer {
    pub fn new(max_size: usize) -> Self {
        Buffer {
            signals: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, signal: Signal) {
        if self.signals.len() >= self.max_size {
            self.signals.pop_back();
        }
        self.signals.push_front(signal);
    }

    pub fn get_recent_signals(&self) -> Vec<Signal> {
        let five_seconds_ago = Utc::now() - Duration::seconds(5);
        self.signals
            .iter()
            .filter(|signal| signal.rx_ts > five_seconds_ago)
            .cloned() // Clone the signal to return owned values
            .collect()
    }
}
