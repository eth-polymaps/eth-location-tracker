const N: f64 = 3.5;

pub fn calculate_distance(rssi: i8, tx: i8) -> f64 {
    let diff = (tx - rssi) as f64;
    10f64.powf(diff / (10.0 * N))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1m_distance() {
        assert_eq!(calculate_distance(-77, -77), 1f64);
    }

    #[test]
    fn test_3m_distance() {
        assert_eq!(calculate_distance(-94, -77), 3.0599496f64);
    }
}
