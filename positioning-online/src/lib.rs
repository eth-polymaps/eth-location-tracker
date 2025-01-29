mod http;

use crate::http::HttpClient;
use crossbeam_channel::{Receiver, Sender, select};
use log::{error, info};
use positioning::beacon::Room;
use positioning::geographic::Position;
use positioning::signal::Signal;
use std::thread;
use std::thread::JoinHandle;

pub struct Locator {
    service_key: String,
    service_client_id: String,
    service_endpoint: String,
}

impl Locator {
    pub fn new(service_key: &str, service_client_id: &str, service_endpoint: &str) -> Self {
        Self {
            service_key: service_key.to_string(),
            service_client_id: service_client_id.to_string(),
            service_endpoint: service_endpoint.to_string(),
        }
    }

    pub fn start(
        self,
        rx: Receiver<Vec<Signal>>,
        tx: Sender<(Position, Room)>,
    ) -> anyhow::Result<JoinHandle<()>> {
        let handle = thread::Builder::new()
            .name("online positioning".to_string())
            .stack_size(8 * 1024) // 8 KB stack
            .spawn(move || {
                let mut location_svc = HttpClient::new(
                    &self.service_endpoint,
                    &self.service_key,
                    &self.service_client_id,
                );

                loop {
                    select! {
                    recv(rx) -> msg => match msg {
                        Ok(m) => {
                             match location_svc.publish(m) {
                                Ok((pos, room)) => {
                                    info!("Received position {:.5},{:.5} from service [{}/{}/{}]", pos.lat, pos.lon, room.building, room.floor, room.room);
                                    if let  Err(e) = tx.send((pos, room)){
                                        error!("Failed to send position to service client: {}", e);
                                    }
                                },
                                Err(e) => error!("Failed to publish measurement: {:?}", e),
                            }
                        }
                        Err(_) => break,
                    }
                }
                }
            })?;

        Ok(handle)
    }
}
