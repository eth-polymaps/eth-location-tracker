use embedded_svc::http::client::Client;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use log::debug;
use positioning::beacon;
use positioning::geographic::Position;
use positioning::signal::Signal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub(crate) struct HttpClient {
    http: Client<EspHttpConnection>,
    hostname: String,
    key: String,
    client_id: String,
}

#[derive(Serialize)]
struct BluetoothBeacon {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "major")]
    pub major: u16,
    #[serde(rename = "minor")]
    pub minor: u16,
    #[serde(rename = "txPower")]
    pub tx_power: i8,
    #[serde(rename = "signalStrength")]
    pub rssi: i8,
}

#[derive(Serialize)]
struct RequestBody {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "bluetoothBeacons")]
    pub bluetooth_beacons: Vec<BluetoothBeacon>,
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    #[serde(rename = "location")]
    pub location: Location,

    #[serde(rename = "indoor")]
    pub indoor: Room,

    #[serde(flatten)]
    extra_fields: HashMap<String, Value>, // captures unknown fields
}

#[derive(Serialize, Deserialize)]
struct Location {
    #[serde(rename = "lat")]
    pub lat: f64,
    #[serde(rename = "lon")]
    pub lon: f64,
}

#[derive(Serialize, Deserialize)]
struct Room {
    #[serde(rename = "building")]
    pub building: String,
    #[serde(rename = "floor")]
    pub floor: String,
    #[serde(rename = "room")]
    pub room: String,
}

impl HttpClient {
    pub fn new(hostname: &str, key: &str, client_id: &str) -> Self {
        let http_client_config = Configuration {
            use_global_ca_store: true,
            crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
            ..Configuration::default()
        };

        let httpconnection = EspHttpConnection::new(&http_client_config).unwrap();
        let http = Client::wrap(httpconnection);

        HttpClient {
            http,
            hostname: hostname.to_string(),
            key: key.to_string(),
            client_id: client_id.to_string(),
        }
    }

    pub fn publish(
        &mut self,
        measurement: Vec<Signal>,
    ) -> Result<(Position, beacon::Room), anyhow::Error> {
        let url = format!("{}/location/v1/positioning", self.hostname);

        let headers = [
            ("accept", "application/json"),
            ("Content-Type", "application/json"),
        ];

        let beacons = measurement
            .iter()
            .map(|sig| BluetoothBeacon {
                uuid: sig.beacon.uuid.clone(),
                major: sig.beacon.major,
                minor: sig.beacon.minor,
                tx_power: sig.tx_power,
                rssi: sig.rssi,
            })
            .collect();

        let req = RequestBody {
            id: self.client_id.clone(),
            key: self.key.clone(),
            bluetooth_beacons: beacons,
        };

        let body_json: String = serde_json::to_string(&req)?;

        debug!("calling api {} with body: {}", url, body_json);

        let mut request = self.http.post(url.as_str(), &headers)?;
        request.connection().write(body_json.as_bytes())?;

        let mut response = request.submit()?;
        if response.status() != 200 {
            return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
        }

        let mut buf = Vec::new();
        let mut chunk = [0u8; 256];
        loop {
            let bytes_read = response.read(&mut chunk)?;
            if bytes_read == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..bytes_read]);
        }

        if buf.is_empty() {
            return Err(anyhow::anyhow!("Empty response received"));
        }

        Ok(serde_json::from_slice::<ResponseBody>(&buf).map_or_else(
            |_| (Position::default(), beacon::Room::default()),
            |res| {
                let room = beacon::Room::new(
                    res.indoor.building.as_str(),
                    res.indoor.floor.as_str(),
                    res.indoor.room.as_str(),
                );
                let pos = Position::new(res.location.lat, res.location.lon);
                (pos, room)
            },
        ))
    }
}
