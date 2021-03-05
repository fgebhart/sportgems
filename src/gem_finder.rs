use crate::climb;
use crate::dtypes;
use crate::math;
use crate::velocity;

pub struct InputData {
    pub desired_distance: u32,
    pub coordinates: Vec<(f64, f64)>,
    pub times: dtypes::Times,
    pub distances: dtypes::Distances,
    pub altitudes: dtypes::Altitudes,
}
#[derive(Debug, PartialEq)]
pub enum InputDataError {
    InconsistentLength,
    TooFewDataPoints,
    DistanceTooSmall,
}

impl InputData {
    pub fn new(
        desired_distance: u32,
        coordinates: Vec<(f64, f64)>,
        times: Vec<f64>,
        altitudes: Option<Vec<f64>>,
    ) -> InputData {
        InputData {
            desired_distance,
            coordinates,
            times: dtypes::Times { values: times },
            distances: dtypes::Distances { values: vec![] },
            altitudes: dtypes::Altitudes {
                values: altitudes.unwrap_or(vec![]),
            },
        }
    }

    pub fn find_fastest_section(&mut self) -> dtypes::TargetSection {
        self._compute_vector_of_distances();
        match self._generic_data_checks() {
            Ok(_) => return self._search_section(velocity::_update_sections_max_velocity),
            Err(_) => return dtypes::TargetSection::default(),
        }
    }

    pub fn find_best_climb_section(&mut self) -> dtypes::TargetSection {
        self._compute_vector_of_distances();
        match self._generic_data_checks() {
            Ok(_) => match climb::specific_data_check(self) {
                Ok(_) => return self._search_section(climb::update_sections_max_climb),
                Err(_) => return dtypes::TargetSection::default(),
            },
            Err(_) => return dtypes::TargetSection::default(),
        }
    }

    fn _generic_data_checks(&self) -> Result<(), InputDataError> {
        let total_distance = self.distances.values.last().unwrap().clone();
        if self.coordinates.len() < 2 {
            return Err(InputDataError::TooFewDataPoints);
        } else if self.coordinates.len() != self.times.values.len() {
            return Err(InputDataError::InconsistentLength);
        } else if self.desired_distance as f64 > total_distance {
            return Err(InputDataError::DistanceTooSmall);
        } else {
            return Ok(());
        }
    }

    fn _compute_vector_of_distances(&mut self) {
        let mut distance: f64 = 0.0;
        self.distances.values.push(distance);

        // loop through coordinates and calculate the distance from one coordinate to the next one
        for i in 0..self.coordinates.len() - 1 {
            let coordinate = dtypes::Coordinate {
                lat: self.coordinates[i].0,
                lon: self.coordinates[i].1,
            };
            let next_coordinate = dtypes::Coordinate {
                lat: self.coordinates[i + 1].0,
                lon: self.coordinates[i + 1].1,
            };
            distance += math::calculate_distance(coordinate, next_coordinate);
            self.distances.values.push(distance);
        }
    }
    // implementation of the search algorithm, takes an update func (which depends on the use case) as input argument
    fn _search_section(
        &mut self,
        update_func: fn(&InputData, &mut dtypes::WindowSection, &mut dtypes::TargetSection),
    ) -> dtypes::TargetSection {
        let mut window_sec = dtypes::WindowSection::default();
        let mut target_sec = dtypes::TargetSection::default();
        while window_sec.end < self.distances.values.len() as u32 - 1 {
            println!("{:?}", window_sec);
            if window_sec.distance < self.desired_distance as f64 {
                // build up section to get closer to the desired length of desired_distance
                window_sec.end += 1;
            }
            update_func(&self, &mut window_sec, &mut target_sec);

            // now move the start index further, but ensure that start index does not overtake end index
            if window_sec.distance >= self.desired_distance as f64 {
                if window_sec.start < window_sec.end {
                    window_sec.start += 1;
                } else {
                    window_sec.end += 1;
                }
            }
        }
        // after the while loop is finished, check that found fastest_section is valid and return
        if target_sec.target_value == 0.0 || target_sec.start == target_sec.end {
            println!("no valid section found, probably due to poor input data quality");
            dtypes::TargetSection::default()
        } else {
            target_sec.valid = true;
            target_sec
        }
    }
}

#[cfg(test)]
mod test_gem_finder {
    use super::*;

    #[test]
    fn test_finder_initialization() {
        let finder = InputData::new(
            10_000,
            vec![(48.0, 8.0), (48.0, 8.1)],
            vec![123.4, 124.6],
            None,
        );
        assert_eq!(finder.desired_distance, 10_000);
        assert_eq!(finder.coordinates, vec!((48.0, 8.0), (48.0, 8.1)));
        assert_eq!(finder.times.values, vec!(123.4, 124.6));
    }

    #[test]
    fn test_compute_vector_of_distances() {
        let mut finder = InputData::new(
            10_000,
            vec![(48.0, 8.0), (48.0, 8.1)],
            vec![123.4, 124.6],
            None,
        );

        finder._compute_vector_of_distances();
        assert_eq!(finder.distances.values, vec!(0.0, 7448.684105664539));
    }
}

#[cfg(test)]
mod test_generic_data_checks {
    use super::*;

    #[test]
    fn test_generic_data_checks_too_few_data_points() {
        // generate data with only one data point and assert that the TooFewDataPoints error is returned
        let mut finder = InputData::new(1_000, vec![(48.123, 9.38)], vec![1608228940.8], None);
        finder._compute_vector_of_distances();
        assert_eq!(
            finder._generic_data_checks(),
            Err(InputDataError::TooFewDataPoints)
        );
    }

    #[test]
    fn test_generic_data_checks_distance_too_small() {
        // generate data where the overall distance is smaller than the desired
        // distance and assert that the DistanceTooSmall error is returned
        let mut finder = InputData::new(
            10_000,
            vec![(48.123, 9.380), (48.123, 9.381)],
            vec![1608228940.8, 1608228941.8],
            None,
        );
        finder._compute_vector_of_distances();
        assert_eq!(
            finder._generic_data_checks(),
            Err(InputDataError::DistanceTooSmall)
        );
    }

    #[test]
    fn test_generic_data_checks_inconsistent_length() {
        // generate data where the coordinates and times vector have different
        // lengths and assert that the InconsistentLength error is returned
        let mut finder = InputData::new(
            10_000,
            vec![(48.123, 9.380), (48.123, 9.381), (48.123, 9.382)],
            vec![1608228940.8, 1608228941.8],
            None,
        );
        finder._compute_vector_of_distances();
        assert_eq!(
            finder._generic_data_checks(),
            Err(InputDataError::InconsistentLength)
        );
    }
}

#[cfg(test)]
mod test_find_fastest_section {
    use super::*;
    #[test]
    fn test_find_fastest_section_edge_case_no_change_in_time() {
        // test case where coordinates are changing but time does not, this would lead to infinite velocity
        let mut finder = InputData::new(
            1_000,
            vec![(48.0, 8.0), (48.0, 8.1)],
            vec![123.4, 123.4],
            None,
        );

        // in this scenario we expect no valid section to be found
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section, dtypes::TargetSection::default());
    }

    #[test]
    fn test_find_fastest_section_dummy_values() {
        // add test with dummy values
        let mut finder = InputData::new(
            1_000,
            vec![
                (48.123, 9.35),
                (48.123, 9.36),
                (48.123, 9.37),
                (48.123, 9.38),
            ],
            vec![1608228953.8, 1608228954.8, 1608228955.8, 1608228956.8],
            None,
        );

        // in this scenario we expect a valid result section
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 0);
        assert_eq!(fastest_section.end, 1);
        assert_eq!(fastest_section.target_value.round(), 743.0);
    }
    #[test]
    fn test_find_fastest_section_nan_values() {
        // add test with null values
        let mut finder = InputData::new(
            1_000,
            vec![
                (f64::NAN, f64::NAN),
                (48.123, 9.36),
                (48.123, 9.37),
                (48.123, 9.38),
            ],
            vec![1608228940.8, 1608228950.8, 1608228960.8, 1608228970.8],
            None,
        );

        // in this scenario we expect a valid result section
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 0);
        assert_eq!(fastest_section.end, 2);
        assert_eq!(fastest_section.target_value.round(), 37.0);
    }

    #[test]
    fn test_find_fastest_section_larger_data() {
        // add test with more values
        let mut finder = InputData::new(
            250,
            vec![
                (48.0001, 9.001),
                (48.0002, 9.002),
                (48.0003, 9.003),
                (48.0006, 9.004), // increase distance here
                (48.0009, 9.005),
                (48.0012, 9.006),
                (48.0015, 9.007), // return back to lower pace here again
                (48.0016, 9.008),
                (48.0017, 9.009),
                (48.0018, 9.010),
            ],
            vec![
                1608228950.8,
                1608228961.8,
                1608228972.8,
                1608228983.8,
                1608228994.8,
                1608229005.8,
                1608229016.8,
                1608229027.8,
                1608229038.8,
                1608229049.8,
            ],
            None,
        );

        // in this scenario we expect a valid result section
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 2); // at index 2 the step distance increases
        assert_eq!(fastest_section.end, 5);
        assert_eq!(fastest_section.target_value.round(), 7.0);
    }
}
