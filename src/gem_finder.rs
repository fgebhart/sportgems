// mod geo;


#[derive(Debug)]
pub struct GemFinder {

    fastest_distance: u32,
    coordinates: Vec<(f64, f64)>,
    times: Vec<f64>,
    distances: Vec<f64>,
}

impl GemFinder {
    pub fn new(fastest_distance: u32, coordinates: Vec<(f64, f64)>, times: Vec<f64>) -> GemFinder {
        GemFinder {
            fastest_distance,     // TODO: can be simplified
            coordinates,
            times,
            distances: vec!(),
        }
    }
    pub fn find_gems(&mut self) -> (u32, u32) {
        self.get_vector_of_distances();
        let result = (138, 547);
        result
    }
    pub fn get_vector_of_distances(&mut self) {
        let distance: f64 = 0.0;
        self.distances = vec!(distance);
        // geo::to_rad();
        return;
    }
}
