use crate::beacon::Id;
use chrono::{DateTime, Duration, Utc};
use crossbeam_channel::{Receiver, Sender, select, tick};
use log::{error, info};
use std::collections::VecDeque;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug, Clone)]
pub struct Signal {
    pub beacon: Id,
    pub tx_power: i8,
    pub rssi: i8,
    pub rx_ts: DateTime<Utc>,
}

impl Signal {
    pub fn new(beacon: Id, tx_power: i8, rssi: i8) -> Self {
        Signal {
            beacon,
            tx_power,
            rssi,
            rx_ts: Utc::now(),
        }
    }
}

#[derive(Default)]
pub struct Processor {}

impl Processor {
    pub fn start(
        &self,
        rx_bluetooth: Receiver<Signal>,
        tx_signals: Sender<Vec<Signal>>,
    ) -> JoinHandle<()> {
        thread::Builder::new()
            .name("processor".to_string())
            .stack_size(8 * 1024) // 8 KB stack
            .spawn(move || {
                let mut buffer = Buffer::new(20);
                let ticker = tick(std::time::Duration::from_secs(5));

                loop {
                    select! {
                        recv(rx_bluetooth) -> signal => match signal {
                            Ok(m) => {
                                info!("received {:?}", m);
                                buffer.push(m);
                            }
                            Err(e) => error!("error receiving signal: {:?}", e),
                        },

                        recv(ticker) -> _ => {
                            if let Err(e) =  tx_signals.send(buffer.get_recent_signals()){
                                error!("error sending signals: {:?}", e);
                            }
                        }
                    }
                }
            })
            .expect("cannot spawn display updater thread")
    }
}

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
