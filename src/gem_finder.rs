use crate::geo;

#[derive(PartialEq, Debug)]
pub struct ResultSection {
    pub valid: bool,
    pub start: u32,
    pub end: u32,
    pub velocity: f64,
}

pub const INVALID_FASTEST_SECTION: ResultSection = ResultSection {
    valid: false,
    start: 0,
    end: 0,
    velocity: 0.0,
};

#[derive(Debug, Clone)]
pub struct Section {
    pub start: u32,
    pub end: u32,
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

pub fn update_section(distances: &geo::Distances, times: &geo::Times, section: &mut Section) {
    section.distance =
        distances.values[section.end as usize] - distances.values[section.start as usize];
    section.duration = times.values[section.end as usize] - times.values[section.start as usize];
    section.velocity = get_velocity(section.distance, section.duration);
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
            println!("no valid section found: desired fastest distance is greater than the total distance of the activity");
            return INVALID_FASTEST_SECTION;
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
            start: 0,
            end: 0,
            distance: 0.0,
            duration: 0.0,
            velocity: 0.0,
        };
        let mut fastest_sec: Section = Section {
            start: 0,
            end: 0,
            distance: 0.0,
            duration: 0.0,
            velocity: 0.0,
        };
        while curr_sec.end < self.distances.values.len() as u32 - 1 {
            if curr_sec.distance < self.fastest_distance as f64 {
                // build up section to get closer to the desired length of fastest_distance
                curr_sec.end += 1;
                update_section(&self.distances, &self.times, &mut curr_sec);

                // update fastest section only in case the current
                // distance is not larger than the required distance + 1%
                if curr_sec.distance <= (self.fastest_distance as f64) * 1.01 {
                    if curr_sec.velocity > fastest_sec.velocity {
                        fastest_sec = curr_sec.clone();
                    }
                }
            } else {
                update_section(&self.distances, &self.times, &mut curr_sec);

                // update fastest section only in case the current
                // distance is not larger than the required distance + 1%
                if curr_sec.distance <= (self.fastest_distance as f64) * 1.01 {
                    if curr_sec.velocity > fastest_sec.velocity {
                        fastest_sec = curr_sec.clone();
                    }
                }
                // now move the start index further, but ensure that start index does not overtake end index
                if curr_sec.start < curr_sec.end {
                    curr_sec.start += 1;
                } else {
                    curr_sec.end += 1;
                }
            }
            println!("{:?}", curr_sec);
        }
        if fastest_sec.velocity == 0.0 || fastest_sec.start == fastest_sec.end {
            println!("no valid section found: poor input data quality");
            INVALID_FASTEST_SECTION
        } else {
            ResultSection {
                valid: true,
                start: fastest_sec.start,
                end: fastest_sec.end,
                velocity: fastest_sec.velocity,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 0);
        assert_eq!(fastest_section.end, 1);
        assert_eq!(fastest_section.velocity.round(), 743.0);
    }
    #[test]
    fn test_find_fastest_section_nan_values() {
        // add test with dummy values
        let mut finder = GemFinder::new(
            1_000,
            vec![
                (f64::NAN, f64::NAN),
                (48.123, 9.36),
                (48.123, 9.37),
                (48.123, 9.38),
            ],
            vec![1608228940.8, 1608228950.8, 1608228960.8, 1608228970.8],
        );

        // in this scenario we expect a valid result section
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 0);
        assert_eq!(fastest_section.end, 2);
        assert_eq!(fastest_section.velocity.round(), 37.0);
    }
}
