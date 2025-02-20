use crate::offline::trilateration::trilaterate;
use log::{error, info};
use crate::beacon::{Beacon, BeaconId, Output, Room};
use crate::geographic::Position;
use crate::signal::Signal;

#[derive(Default)]
pub struct Locator {}

impl Locator {
    pub(crate) fn locate(&self, signals: Vec<Signal<BeaconId>>) -> anyhow::Result<Output> {
        let resolved_signals = Self::resolve_beacons(signals);
        let distances_signals = Self::calculate_signal_distance(resolved_signals);

        if let Some(first) = distances_signals.first() {
            distances_signals
                .iter()
                .for_each(|x| info!("calculated distance to beacon {:?}", x));

            let measurements: Vec<_> = distances_signals
                .iter()
                .filter_map(|s| {
                    s.distance.map(|d| {
                        super::trilateration::Measurement::new(
                            s.beacon.position.lat,
                            s.beacon.position.lon,
                            d,
                        )
                    })
                })
                .collect();

            // Perform trilateration and return result
            Ok(Output::new(
                trilaterate(measurements)?,
                first.beacon.location.clone(),
                None,
                None,
            ))
        } else {
            Err(anyhow::anyhow!("did not find any signals"))
        }
    }

    fn resolve_beacons(signals: Vec<Signal<BeaconId>>) -> Vec<Signal<Beacon>> {
        signals
            .iter()
            .flat_map(|s| {
                let resolved_beacon = eth_beacons_indoor::resolver::find_beacon_by_id(
                    s.beacon.uuid.as_str(),
                    s.beacon.major,
                    s.beacon.minor,
                );

                if resolved_beacon.is_none() {
                    error!(
                        "beacon for uuid {}, major {}, minor {} not found",
                        s.beacon.uuid.as_str(),
                        s.beacon.major,
                        s.beacon.minor
                    );
                }

                resolved_beacon.map(|b| {
                    let id = BeaconId::new(b.id.uuid, b.id.major, b.id.minor);
                    let loc = &b.location;
                    let location = Room::new(loc.building.as_ref(), loc.floor, loc.room);
                    let position = Position::new(b.position.lat, b.position.lon);

                    Signal::new(Beacon::new(id, location, position), s.tx_power, s.rssi)
                })
            })
            .collect()
    }

    fn calculate_signal_distance(signals: Vec<Signal<Beacon>>) -> Vec<Signal<Beacon>> {
        let mut result = signals
            .iter()
            .map(move |s| {
                let distance = super::signal::calculate_distance(s.rssi, s.tx_power);
                s.clone().with_distance(distance) // Clone `s` before modifying it
            })
            .filter(|d| d.distance.is_some())
            .collect::<Vec<Signal<Beacon>>>();

        result.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        result
    }
}
