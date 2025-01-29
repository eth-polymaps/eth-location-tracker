use crossbeam_channel::{Receiver, select};
use log::info;
use positioning::signal::Signal;
use std::thread;
use std::thread::JoinHandle;

#[derive(Default)]
pub struct Locator {}

impl Locator {
    pub fn start(self, rx: Receiver<Vec<Signal>>) -> JoinHandle<()> {
        thread::Builder::new()
            .name("locator".to_string())
            .stack_size(8 * 1024) // 8 KB stack
            .spawn(move || {
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
            .expect("cannot spawn locator thread")
    }
}
