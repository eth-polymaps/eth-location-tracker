use crate::beacon::Id;
use chrono::{DateTime, Utc};
use crossbeam_channel::{select, tick, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use log::{error, info};
use crate::buffer::Buffer;

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

pub struct Processor {}

impl Processor {
    pub fn new() -> Self {
        Self {}
    }
}


impl Processor {
    pub fn start(&self, rx_bluetooth: Receiver<Signal>, tx_signals: Sender<Vec<Signal>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut buffer = Buffer::new(20);
            let ticker = tick(Duration::from_secs(5));

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
    }
}
