use crate::climb;
use crate::dtypes;
use crate::geo;
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
    pub fn new(desired_distance: u32, coordinates: Vec<(f64, f64)>, times: Vec<f64>, altitudes: Option<Vec<f64>>) -> InputData {
        InputData {
            desired_distance,
            coordinates,
            times: dtypes::Times { values: times },
            distances: dtypes::Distances { values: vec![] },
            altitudes: dtypes::Altitudes { values: altitudes.unwrap_or(vec![]) },
        }
    }

    pub fn find_fastest_section(&mut self) -> Result<dtypes::TargetSection, InputDataError> {
        self._compute_vector_of_distances();
        match self._generic_data_checks() {
            Ok(_) => return Ok(self._search_section(velocity::_update_sections_max_velocity)),
            Err(e) => Err(e),
        }
    }

    pub fn find_best_climb_section(&mut self) -> Result<dtypes::TargetSection, InputDataError> {
        self._compute_vector_of_distances();
        let res = self._generic_data_checks();
        if res.is_ok() {
            let check = climb::specific_data_check(self);
            if check.is_ok() {
                Ok(self._search_section(climb::update_sections_max_climb))
            } else {
                Err(check.unwrap_err())
            }
        } else {
            Err(res.unwrap_err())
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
            distance += geo::calculate_distance(coordinate, next_coordinate);
            self.distances.values.push(distance);
        }
    }
    // implementation of the search algorithm, takes an update func (which depends on the use case) as input argument
    fn _search_section(
        &mut self,
        update_func: fn(
            &InputData,
            &dtypes::Times,
            &mut dtypes::WindowSection,
            &mut dtypes::TargetSection,
        ),
    ) -> dtypes::TargetSection {
        let mut window_sec = dtypes::WindowSection::default();
        let mut target_sec = dtypes::TargetSection::default();
        while window_sec.end < self.distances.values.len() as u32 - 1 {
            // println!("{:?}", curr_sec);
            if window_sec.distance < self.desired_distance as f64 {
                // build up section to get closer to the desired length of desired_distance
                window_sec.end += 1;
            }
            update_func(&self, &self.times, &mut window_sec, &mut target_sec);

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
            println!("no valid section found: poor input data quality");
            dtypes::TargetSection::default()
        } else {
            target_sec.valid = true;
            target_sec
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_fastest_section_initialization() {
        let finder = InputData::new(10_000, vec![(48.0, 8.0), (48.0, 8.1)], vec![123.4, 124.6], None);
        assert_eq!(finder.desired_distance, 10_000);
        assert_eq!(finder.coordinates, vec!((48.0, 8.0), (48.0, 8.1)));
        assert_eq!(finder.times.values, vec!(123.4, 124.6));
    }

    #[test]
    fn test_compute_vector_of_distances() {
        let mut finder = InputData::new(10_000, vec![(48.0, 8.0), (48.0, 8.1)], vec![123.4, 124.6], None);

        finder._compute_vector_of_distances();
        assert_eq!(finder.distances.values, vec!(0.0, 7448.684105664539));
    }

    #[test]
    fn test_find_fastest_section_edge_case_no_change_in_time() {
        // test case where coordinates are changing but time does not, this would lead to infinite velocity
        let mut finder = InputData::new(1_000, vec![(48.0, 8.0), (48.0, 8.1)], vec![123.4, 123.4], None);

        // in this scenario we expect no valid section to be found
        let fastest_section = finder.find_fastest_section();
        assert_eq!(fastest_section.unwrap(), dtypes::TargetSection::default());
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
            vec![1608228953.8, 1608228954.8, 1608228955.8, 1608228956.8], None,
        );

        // in this scenario we expect a valid result section
        let fastest_section = finder.find_fastest_section().unwrap();
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 0);
        assert_eq!(fastest_section.end, 1);
        assert_eq!(fastest_section.target_value.round(), 743.0);
    }
    #[test]
    fn test_find_fastest_section_nan_values() {
        // add test with dummy values
        let mut finder = InputData::new(
            1_000,
            vec![
                (f64::NAN, f64::NAN),
                (48.123, 9.36),
                (48.123, 9.37),
                (48.123, 9.38),
            ],
            vec![1608228940.8, 1608228950.8, 1608228960.8, 1608228970.8], None,
        );

        // in this scenario we expect a valid result section
        let fastest_section = finder.find_fastest_section().unwrap();
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 0);
        assert_eq!(fastest_section.end, 3);
        assert_eq!(fastest_section.target_value.round(), 50.0);
    }

    //
    // test errors of _generic_data_checks
    //
    #[test]
    fn test_generic_data_checks_too_few_data_points() {
        // generate data with only one data point and assert that the TooFewDataPoints error is returned
        let mut finder = InputData::new(1_000, vec![(48.123, 9.38)], vec![1608228940.8], None,);
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
            vec![1608228940.8, 1608228941.8], None,
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
            vec![1608228940.8, 1608228941.8], None,
        );
        finder._compute_vector_of_distances();
        assert_eq!(
            finder._generic_data_checks(),
            Err(InputDataError::InconsistentLength)
        );
    }

    //
    // test errors of find_best_climb_section
    //
    #[test]
    fn test_find_best_climb_section_too_few_data_points() {
        // assert that the TooFewDataPoints error is returned
        let mut finder = InputData::new(1_000, vec![(48.123, 9.38)], vec![1608228940.8], None,);
        assert_eq!(
            finder.find_best_climb_section(),
            Err(InputDataError::TooFewDataPoints)
        );
    }

    #[test]
    fn test_find_best_climb_section_distance_too_small() {
        // generate data where the overall distance is smaller than the desired
        // distance and assert that the DistanceTooSmall error is returned
        let mut finder = InputData::new(
            10_000,
            vec![(48.123, 9.380), (48.123, 9.381)],
            vec![1608228940.8, 1608228941.8], None,
        );
        assert_eq!(
            finder.find_best_climb_section(),
            Err(InputDataError::DistanceTooSmall)
        );
    }

    #[test]
    fn test_find_best_climb_section_inconsistent_length() {
        // generate data where the coordinates and times vector have different
        // lengths and assert that the InconsistentLength error is returned
        let mut finder = InputData::new(
            10_000,
            vec![(48.123, 9.380), (48.123, 9.381), (48.123, 9.382)],
            vec![1608228940.8, 1608228941.8], None,
        );
        assert_eq!(
            finder.find_best_climb_section(),
            Err(InputDataError::InconsistentLength)
        );
    }

    #[test]
    fn test_find_best_climb_section_inconsistent_length_altitude() {
        // coordinates and time are with consistent length but altitude vector not
        let mut finder = InputData::new(
            1_000,
            vec![(48.123, 9.38), (48.123, 9.39), (48.123, 9.40)],
            vec![1608228940.8, 1608228941.8, 1608228942.8], Some(vec![123.4, 234.5]),
        );
        assert_eq!(
            finder.find_best_climb_section(),
            Err(InputDataError::InconsistentLength)
        );
    }
    #[test]
    fn test_find_best_climb_section_distance_too_small_altitude() {
        // coordinates and times are two of length 2 but altitudes is too short
        let mut finder = InputData::new(
            1_000,
            vec![(48.123, 9.39), (48.123, 9.40)],
            vec![1608228940.8, 1608228941.8], Some(vec![456.7]),
        );
        assert_eq!(
            finder.find_best_climb_section(),
            Err(InputDataError::DistanceTooSmall)
        );
    }

    //
    // test errors of find_fastest_section
    //
    #[test]
    fn test_find_fastest_section_too_few_data_points() {
        // assert that the TooFewDataPoints error is returned
        let mut finder = InputData::new(1_000, vec![(48.123, 9.38)], vec![1608228940.8], None,);
        assert_eq!(
            finder.find_fastest_section(),
            Err(InputDataError::TooFewDataPoints)
        );
    }

    #[test]
    fn test_find_fastest_section_distance_too_small() {
        // generate data where the overall distance is smaller than the desired
        // distance and assert that the DistanceTooSmall error is returned
        let mut finder = InputData::new(
            10_000,
            vec![(48.123, 9.380), (48.123, 9.381)],
            vec![1608228940.8, 1608228941.8], None,
        );
        assert_eq!(
            finder.find_fastest_section(),
            Err(InputDataError::DistanceTooSmall)
        );
    }

    #[test]
    fn test_find_fastest_section_inconsistent_length() {
        // generate data where the coordinates and times vector have different
        // lengths and assert that the InconsistentLength error is returned
        let mut finder = InputData::new(
            10_000,
            vec![(48.123, 9.380), (48.123, 9.381), (48.123, 9.382)],
            vec![1608228940.8, 1608228941.8], None,
        );
        assert_eq!(
            finder.find_fastest_section(),
            Err(InputDataError::InconsistentLength)
        );
    }
}
