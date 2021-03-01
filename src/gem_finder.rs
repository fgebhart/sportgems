use crate::geo;
use std::default::Default;

#[derive(PartialEq, Debug)]
pub struct FastestSection {
    pub valid: bool,
    pub start: u32,
    pub end: u32,
    pub velocity: f64,
}

impl Default for FastestSection {
    fn default() -> FastestSection {
        FastestSection {
            valid: false,
            start: 0,
            end: 0,
            velocity: 0.0,
        }
    }
}

pub const INVALID_FASTEST_SECTION: FastestSection = FastestSection {
    valid: false,
    start: 0,
    end: 0,
    velocity: 0.0,
};

#[derive(Debug, Clone)]
pub struct WindowSection {
    pub start: u32,
    pub end: u32,
    pub distance: f64,
    pub duration: f64,
    pub velocity: f64,
}

impl Default for WindowSection {
    fn default() -> WindowSection {
        WindowSection {
            start: 0,
            end: 0,
            distance: 0.0,
            duration: 0.0,
            velocity: 0.0,
        }
    }
}

pub struct GemFinder {
    pub desired_distance: u32,
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

pub fn update_current_section(
    distances: &geo::Distances,
    times: &geo::Times,
    section: &mut WindowSection,
) {
    section.distance =
        distances.values[section.end as usize] - distances.values[section.start as usize];
    section.duration = times.values[section.end as usize] - times.values[section.start as usize];
    section.velocity = get_velocity(section.distance, section.duration);
}

pub fn update_fastest_section(
    desired_distance: f64,
    current_section: &WindowSection,
    fastest_section: &mut FastestSection,
) {
    // update fastest section only in case the current
    // distance is not larger than the required distance + 1%
    if current_section.distance <= (desired_distance) * 1.01 {
        if current_section.velocity > fastest_section.velocity {
            fastest_section.start = current_section.start;
            fastest_section.end = current_section.end;
            fastest_section.velocity = current_section.velocity;
        }
    }
}

impl GemFinder {
    pub fn new(desired_distance: u32, coordinates: Vec<(f64, f64)>, times: Vec<f64>) -> GemFinder {
        GemFinder {
            desired_distance,
            coordinates,
            times: geo::Times { values: times },
            distances: geo::Distances { values: vec![] },
        }
    }
    pub fn find_fastest_section(&mut self) -> FastestSection {
        assert!(
            self.coordinates.len() == self.times.values.len(),
            "Length of coordinates and times must be equal."
        );
        self.compute_vector_of_distances();
        let total_distance = self.distances.values.last().unwrap().clone();
        if self.desired_distance as f64 > total_distance {
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
    pub fn search_section(&mut self) -> FastestSection {
        let mut curr_sec = WindowSection::default();
        let mut fastest_sec = FastestSection::default();
        while curr_sec.end < self.distances.values.len() as u32 - 1 {
            // println!("{:?}", curr_sec);
            if curr_sec.distance < self.desired_distance as f64 {
                // build up section to get closer to the desired length of desired_distance
                curr_sec.end += 1;
            }
            update_current_section(&self.distances, &self.times, &mut curr_sec);
            update_fastest_section(self.desired_distance as f64, &curr_sec, &mut fastest_sec);
            // now move the start index further, but ensure that start index does not overtake end index
            if curr_sec.distance >= self.desired_distance as f64 {
                if curr_sec.start < curr_sec.end {
                    curr_sec.start += 1;
                } else {
                    curr_sec.end += 1;
                }
            }
        }
        // after the while loop is finished, check that found fastest_section is valid and return
        if fastest_sec.velocity == 0.0 || fastest_sec.start == fastest_sec.end {
            println!("no valid section found: poor input data quality");
            INVALID_FASTEST_SECTION
        } else {
            fastest_sec.valid = true;
            fastest_sec
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
        assert_eq!(finder.desired_distance, 10_000);
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
