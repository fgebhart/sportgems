use crate::geo;

#[derive(Debug, Clone)]
pub struct Section {
    pub start_index: u32,
    pub end_index: u32,
    pub distance: f64,
    pub duration: f64,
    pub velocity: f64,
}

pub struct GemFinder {
    pub fastest_distance: u32,
    pub coordinates: Vec<(f64, f64)>,
    pub times: geo::Times,
    pub distances: geo::Distances,
}

impl GemFinder {
    pub fn new(fastest_distance: u32, coordinates: Vec<(f64, f64)>, times: Vec<f64>) -> GemFinder {
        GemFinder {
            fastest_distance,
            coordinates,
            times: geo::Times { values: times },
            distances: geo::Distances { values: vec![] },
        }
    }
    pub fn find_gems(&mut self) -> (u32, u32, f64) {
        assert!(
            self.coordinates.len() == self.times.values.len(),
            "Length of coordinates and times must be equal."
        );
        self.compute_vector_of_distances();
        let total_distance = self.distances.values.last().unwrap().clone();
        if self.fastest_distance as f64 > total_distance {
            return (0, 0, 0.0);
        } else {
            self.search_section()
        }
    }
    pub fn compute_vector_of_distances(&mut self) {
        let mut distance: f64 = 0.0;
        self.distances.values.push(distance);

        // loop through coordinates and calculate the distance from one coordinate to the next one
        for i in 0..self.coordinates.len() - 1 {
            let coordinate = geo::Coordinate {
                lat: self.coordinates[i].0,
                lon: self.coordinates[i].1,
            };
            let next_coordinate = geo::Coordinate {
                lat: self.coordinates[i + 1].0,
                lon: self.coordinates[i + 1].1,
            };
            distance += geo::calculate_distance(coordinate, next_coordinate);
            self.distances.values.push(distance);
        }
    }
    pub fn search_section(&mut self) -> (u32, u32, f64) {
        let mut curr_sec: Section = Section {
            start_index: 0,
            end_index: 0,
            distance: 0.0,
            duration: 0.0,
            velocity: 0.0,
        };
        let mut fastest_sec: Section = Section {
            start_index: 0,
            end_index: 0,
            distance: 0.0,
            duration: 0.0,
            velocity: 0.0,
        };
        while curr_sec.end_index < self.distances.values.len() as u32 - 1 {
            println!("{:?}", curr_sec);
            if curr_sec.distance < self.fastest_distance as f64 {
                // build up section to have length of fastest_distance
                curr_sec.end_index += 1;
                curr_sec.distance = self.distances.values[curr_sec.end_index as usize]
                    - self.distances.values[curr_sec.start_index as usize]
            } else {
                // update section distance, duration and velocity
                curr_sec.distance = self.distances.values[curr_sec.end_index as usize]
                    - self.distances.values[curr_sec.start_index as usize];
                curr_sec.duration = self.times.values[curr_sec.end_index as usize]
                    - self.times.values[curr_sec.start_index as usize];
                curr_sec.velocity = curr_sec.distance / curr_sec.duration;
                // update fastest section only in case the current
                // distance is not larger than the required distance + 1%
                if curr_sec.distance <= (self.fastest_distance as f64) * 1.01 {
                    if curr_sec.velocity > fastest_sec.velocity {
                        fastest_sec = curr_sec.clone();
                    }
                }
                // now move the start index further, end index will also
                // be moved if section gets smaller than fastest_distance
                curr_sec.start_index += 1;
            }
        }
        (
            fastest_sec.start_index,
            fastest_sec.end_index,
            fastest_sec.velocity,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gem_finder_initialization() {
        let mut finder = GemFinder::new(10000, vec![(48.0, 8.0), (48.0, 8.1)], vec![123.4, 124.6]);
        assert_eq!(finder.fastest_distance, 10000);
        assert_eq!(finder.coordinates, vec!((48.0, 8.0), (48.0, 8.1)));
        assert_eq!(finder.times.values, vec!(123.4, 124.6));

        // now compute the distances
        finder.compute_vector_of_distances();
        assert_eq!(finder.distances.values, vec!(0.0, 7448.684105664539));

        // test case where fastest distance is greater than the
        // total distance, see above: 10000 > 7448
        let gem = finder.find_gems();
        assert_eq!(gem, (0, 0, 0.0));
    }
}
