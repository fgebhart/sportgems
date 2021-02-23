use crate::geo;

#[derive(PartialEq, Debug)]
pub struct ResultSection {
    pub valid_section: bool,
    pub start_index: u32,
    pub end_index: u32,
    pub velocity: f64,
}

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

pub fn get_velocity(distance: f64, time: f64) -> f64 {
    let velocity: f64 = distance / time;
    if !velocity.is_normal() {
        return 0.0;
    } else {
        return velocity;
    }
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
    pub fn find_fastest_section(&mut self) -> ResultSection {
        assert!(
            self.coordinates.len() == self.times.values.len(),
            "Length of coordinates and times must be equal."
        );
        self.compute_vector_of_distances();
        let total_distance = self.distances.values.last().unwrap().clone();
        if self.fastest_distance as f64 > total_distance {
            return ResultSection {
                valid_section: false,
                start_index: 0,
                end_index: 0,
                velocity: 0.0,
            };
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
    pub fn search_section(&mut self) -> ResultSection {
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
            // println!("{:?}", curr_sec);
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
                curr_sec.velocity = get_velocity(curr_sec.distance, curr_sec.duration);
                // update fastest section only in case the current
                // distance is not larger than the required distance + 1%
                if curr_sec.distance <= (self.fastest_distance as f64) * 1.01 {
                    if curr_sec.velocity > fastest_sec.velocity {
                        fastest_sec = curr_sec.clone();
                    }
                }
                // now move the start index further, but ensure that start index does not overtake end index
                if curr_sec.start_index < curr_sec.end_index {
                    curr_sec.start_index += 1;
                } else {
                    curr_sec.end_index += 1;
                }
            }
        }
        if fastest_sec.velocity == 0.0 || fastest_sec.start_index == fastest_sec.end_index {
            ResultSection {
                valid_section: false,
                start_index: 0,
                end_index: 0,
                velocity: 0.0,
            }
        } else {
            ResultSection {
                valid_section: true,
                start_index: fastest_sec.start_index,
                end_index: fastest_sec.end_index,
                velocity: fastest_sec.velocity,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub const INVALID_FASTEST_SECTION: ResultSection = ResultSection {
        valid_section: false,
        start_index: 0,
        end_index: 0,
        velocity: 0.0,
    };

    #[test]
    fn test_get_velocity() {
        assert_eq!(get_velocity(16.0, 2.0), 8.0);
        assert_eq!(get_velocity(5.0, 1.0), 5.0);
        // division by zero should return zero
        assert_eq!(get_velocity(3.0, 0.0), 0.0);
        // in case either of the inputs is NAN we expect also 0.0
        assert_eq!(get_velocity(f64::NAN, 7.0), 0.0);
        assert_eq!(get_velocity(4.0, f64::NAN), 0.0);
        assert_eq!(get_velocity(f64::NAN, f64::NAN), 0.0);
    }

    #[test]
    fn test_find_fastest_section_initialization() {
        let mut finder = GemFinder::new(10_000, vec![(48.0, 8.0), (48.0, 8.1)], vec![123.4, 124.6]);
        assert_eq!(finder.fastest_distance, 10_000);
        assert_eq!(finder.coordinates, vec!((48.0, 8.0), (48.0, 8.1)));
        assert_eq!(finder.times.values, vec!(123.4, 124.6));

        // now compute the distances
        finder.compute_vector_of_distances();
        assert_eq!(finder.distances.values, vec!(0.0, 7448.684105664539));

        // test case where fastest distance is greater than the
        // total distance, see above: 10000 > 7448
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section, INVALID_FASTEST_SECTION);
    }

    #[test]
    fn test_find_fastest_section_edge_case_no_change_in_time() {
        // test case where coordinates are changing but time does not, this would lead to infinite velocity
        let mut finder = GemFinder::new(1_000, vec![(48.0, 8.0), (48.0, 8.1)], vec![123.4, 123.4]);

        // in this scenario we expect no valid section to be found
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section, INVALID_FASTEST_SECTION);
    }

    #[test]
    fn test_find_fastest_section_edge_case_no_change_in_coordinates() {
        // test case where coordinates are not changing but time does
        let mut finder = GemFinder::new(1_000, vec![(48.0, 8.0), (48.0, 8.0)], vec![123.4, 124.5]);

        // in this scenario we expect no valid section to be found
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section, INVALID_FASTEST_SECTION);
    }

    #[test]
    fn test_find_fastest_section_dummy_values() {
        // add test with dummy values
        let mut finder = GemFinder::new(
            1_000,
            vec![
                (48.123, 9.35),
                (48.123, 9.36),
                (48.123, 9.37),
                (48.123, 9.38),
            ],
            vec![1608228953.8, 1608228954.8, 1608228955.8, 1608228956.8],
        );

        // in this scenario we expect a valid result section
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section.valid_section, true);
        assert_eq!(fastest_section.start_index, 1);
        assert_eq!(fastest_section.end_index, 2);
        assert_eq!(fastest_section.velocity.round(), 743.0);
    }
}
