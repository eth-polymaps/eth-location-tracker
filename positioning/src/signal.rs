use crate::beacon::BeaconId;
use chrono::{DateTime, Duration, Utc};
use crossbeam_channel::{Receiver, Sender, select, tick};
use log::{error, info};
use std::collections::VecDeque;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug, Clone)]
pub struct Signal<T> {
    pub beacon: T,
    pub tx_power: i8,
    pub rssi: i8,
    pub rx_ts: DateTime<Utc>,
    pub distance: Option<f64>,
}

impl<T> Signal<T> {
    pub fn with_distance(self, distance: f64) -> Signal<T> {
        Self {
            distance: Some(distance),
            ..self
        }
    }
}

impl<T: Clone> Signal<T> {
    pub fn new(beacon: T, tx_power: i8, rssi: i8) -> Self {
        Signal {
            beacon,
            tx_power,
            rssi,
            rx_ts: Utc::now(),
            distance: None,
        }
    }
}

#[derive(Default)]
pub struct Processor {}

impl Processor {
    pub fn start(
        &self,
        rx_bluetooth: Receiver<Signal<BeaconId>>,
        tx_signals: Sender<Vec<Signal<BeaconId>>>,
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
                                info!("pushing signal {:?}", m);
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

pub struct Buffer<T: Clone> {
    signals: VecDeque<Signal<T>>, // VecDeque to store the signals
    max_size: usize,
}

impl<T: Clone> Buffer<T> {
    pub fn new(max_size: usize) -> Self {
        Buffer {
            signals: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, signal: Signal<T>) {
        if self.signals.len() >= self.max_size {
            self.signals.pop_back();
        }
        self.signals.push_front(signal);
    }

    pub fn get_recent_signals(&self) -> Vec<Signal<T>> {
        let five_seconds_ago = Utc::now() - Duration::seconds(5);
        self.signals
            .iter()
            .filter(|signal| signal.rx_ts > five_seconds_ago)
            .cloned()
            .collect::<Vec<Signal<T>>>()
    }
}
