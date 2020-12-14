
// const MOVING_PERIOD: f64 = 0.08;
// const RESTART_TIME: f64 = 2.0;

#[derive(Debug)]
pub struct GemFinder {

    fastest_distance: u32,
    coordinates: Vec<(f64, f64)>,
    times: Vec<f64>,
}

impl GemFinder {
    pub fn new(fastest_distance: u32, coordinates: Vec<(f64, f64)>, times: Vec<f64>) -> GemFinder {
        GemFinder {
            fastest_distance: fastest_distance,
            coordinates: coordinates,
            times: times,
        }
    }
}
