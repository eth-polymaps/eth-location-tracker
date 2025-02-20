mod locator;
mod signal;
mod trilateration;

use crossbeam_channel::{Receiver, Sender, select};
use log::error;
use crate::beacon::{BeaconId, Output};
use crate::signal::Signal;
use std::thread;
use std::thread::JoinHandle;

#[derive(Default)]
pub struct Locator {}

impl Locator {
    pub fn start(
        self,
        rx: Receiver<Vec<Signal<BeaconId>>>,
        tx: Sender<Output>,
    ) -> anyhow::Result<JoinHandle<()>> {
        let handle = thread::Builder::new()
            .name("locator".to_string())
            .stack_size(8 * 1024) // 8 KB stack
            .spawn(move || {
                let positioning = locator::Locator::default();

                loop {
                    select! {
                        recv(rx) -> msg => match msg {
                            Ok(m) => {
                             match positioning.locate(m) {
                                Ok(output) => {
                                    if let  Err(e) = tx.send(output){
                                        error!("Failed to send position to service client: {}", e);
                                    }
                                },
                                Err(e) => error!("Failed to publish measurement: {}", e),
                            }
                            }
                            Err(_) => break,
                        }
                    }
                }
            })
            .expect("cannot spawn locator thread");

        Ok(handle)
    }
}
