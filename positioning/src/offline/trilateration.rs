use log::info;
use std::time::Instant;

use argmin::core::{CostFunction, Error, Executor};
use argmin::solver::neldermead::NelderMead;
use crate::geographic::{Position, haversine_distance};

struct Quadratic {
    measurements: Vec<Measurement>,
}

impl Quadratic {
    fn new(measurements: Vec<Measurement>) -> Self {
        Self { measurements }
    }
}

pub(crate) struct Measurement {
    lat: f64,
    lon: f64,
    distance: f64,
    weight: f64,
}

impl Measurement {
    pub fn new(lat: f64, lon: f64, distance: f64) -> Self {
        Self {
            lat,
            lon,
            distance,
            weight: 1.0,
        }
    }
}

impl CostFunction for Quadratic {
    type Param = Vec<f64>;
    type Output = f64;

    fn cost(&self, x: &Self::Param) -> Result<Self::Output, Error> {
        let lat = x[0];
        let lon = x[1];

        let mut sum = 0.0;
        for m in &self.measurements {
            let d = haversine_distance(Position::new(lat, lon), Position::new(m.lat, m.lon));
            let diff = d - m.distance;
            sum += m.weight * diff * diff;
        }

        Ok(sum)
    }
}

pub fn trilaterate(measurements: Vec<Measurement>) -> anyhow::Result<Position> {
    let init_simplex = vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![0.0, 1.0]];
    let solver = NelderMead::new(init_simplex);

    let measurements_length = measurements.len();

    let quadratic = Quadratic::new(measurements); // Use constructor
    let executor = Executor::new(quadratic, solver).configure(|cfg| cfg.max_iters(200));

    let start = Instant::now(); // Start timing
    info!(
        "Starting trilateration with {} measurements.",
        measurements_length
    );
    let res = executor.run()?; // Execute optimization

    let duration = start.elapsed();
    let x = res.state();
    info!("Elapsed time: {:?}", duration);

    info!("Best solution: {:?}", x.best_param);
    info!("Best function value: {:?}", x.best_cost);
    info!("Iterations: {:?} (max={:?})", x.iter, x.max_iters);

    x.best_param
        .as_ref()
        .map(|bp| Position::new(bp[0], bp[1]))
        .ok_or_else(|| anyhow::anyhow!("Optimization failed, no valid parameters found"))
}
