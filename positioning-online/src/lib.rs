mod http;


use positioning::signal::Signal;
use crate::http::HttpClient;
use crossbeam_channel::{select, Receiver};
use log::{error, info};
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

    pub fn start(self, rx: Receiver<Vec<Signal>>) -> JoinHandle<()> {
        thread::spawn(move || {
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
                                Ok(loc) => info!("Successfully published measurement, retrieved location {:.5},{:.5}", loc.lat, loc.lon),
                                Err(e) => error!("Failed to publish measurement: {:?}", e),
                            }
                        }
                        Err(_) => break,
                    }
                }
            }
        })
    }
}
