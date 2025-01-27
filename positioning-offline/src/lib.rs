use positioning::signal::Signal;
use crossbeam_channel::{select, Receiver};
use log::info;
use std::thread;
use std::thread::JoinHandle;

#[derive(Default)]
pub struct Locator {}

impl Locator {
    pub fn start(self, rx: Receiver<Vec<Signal>>) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                select! {
                    recv(rx) -> msg => match msg {
                        Ok(m) => {
                             info!("Received {} signals", m.len());
                        }
                        Err(_) => break,
                    }
                }
            }
        })
    }
}
