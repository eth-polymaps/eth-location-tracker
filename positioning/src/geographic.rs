#[derive(Debug, Clone, Default, Copy)]
pub struct Position {
    pub lat: f64,
    pub lon: f64,
}

impl Position {
    pub fn new(lat: f64, lon: f64) -> Self {
        Position { lat, lon }
    }
}

pub fn euclidean_distance(a: Position, b: Position) -> f64 {
    let x1 = a.lat.to_radians();
    let y1 = a.lon.to_radians();
    let x2 = b.lat.to_radians();
    let y2 = b.lon.to_radians();

    let dx = x2 - x1;
    let dy = y2 - y1;

    (dx.powi(2) + dy.powi(2)).sqrt()
}

pub fn haversine_distance(a: Position, b: Position) -> f64 {
    const R: f64 = 6_371_000.0;

    let lat1_rad = a.lat.to_radians();
    let lat2_rad = b.lat.to_radians();
    let delta_lat = (b.lat - a.lat).to_radians();
    let delta_lon = (b.lon - a.lon).to_radians();

    let x1 = (delta_lat / 2.0).sin();
    let x2 = (delta_lon / 2.0).sin();
    let x = x1 * x1 + lat1_rad.cos() * lat2_rad.cos() * x2 * x2;

    R * 2.0 * x.sqrt().atan2((1.0 - x).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_point() {
        assert_eq!(
            haversine_distance(
                Position::new(47.413310, 8.536444),
                Position::new(47.413310, 8.536444)
            ),
            0f64
        );
    }

    #[test]
    fn test_different_points() {
        assert_eq!(
            haversine_distance(
                Position::new(47.413310, 8.536444),
                Position::new(47.413309, 8.536520)
            ),
            5.719788976313549
        );
    }
}
