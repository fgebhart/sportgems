use crate::dtypes;
use crate::errors;
use crate::math;

const DEFAULT_TOLERANCE: f64 = 0.01;

#[derive(Debug)]
pub struct InputData {
    pub desired_distance: f64,
    pub coordinates: Vec<(f64, f64)>,
    pub times: dtypes::Times,
    pub distances: dtypes::Distances,
    pub altitudes: dtypes::Altitudes,
    pub tolerance: f64,
}

impl InputData {
    pub fn new(
        desired_distance: f64,
        coordinates: Vec<(f64, f64)>,
        times: Vec<f64>,
        altitudes: Option<Vec<f64>>,
        tolerance: Option<f64>,
    ) -> Result<InputData, errors::InputDataError> {
        match _generic_data_checks(coordinates.len(), times.len()) {
            Ok(_) => Ok(InputData {
                desired_distance,
                coordinates,
                times: dtypes::Times { values: times },
                distances: dtypes::Distances { values: vec![] },
                altitudes: dtypes::Altitudes {
                    values: altitudes.unwrap_or(vec![]),
                },
                tolerance: tolerance.unwrap_or(DEFAULT_TOLERANCE),
            }),
            Err(e) => Err(e),
        }
    }

    pub fn _check_if_total_distance_suffice(&self) -> Result<(), errors::InputDataError> {
        let total_distance = self.distances.values.last().unwrap().clone();
        if self.desired_distance > total_distance {
            return Err(errors::InputDataError::DistanceTooSmall);
        } else {
            return Ok(());
        }
    }

    pub fn _compute_vector_of_distances(&mut self) {
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
    pub fn _search_section(
        &mut self,
        update_func: fn(&InputData, &mut dtypes::WindowSection, &mut dtypes::TargetSection),
    ) -> Result<dtypes::TargetSection, errors::InputDataError> {
        let mut window_sec = dtypes::WindowSection::default();
        let mut target_sec = dtypes::TargetSection::default();
        while window_sec.end < self.distances.values.len() as u32 - 1 {
            // println!("{:?}", window_sec);
            if window_sec.distance < self.desired_distance {
                // build up section to get closer to the desired length of desired_distance
                window_sec.end += 1;
            }
            update_func(&self, &mut window_sec, &mut target_sec);

            // now move the start index further, but ensure that start index does not overtake end index
            if window_sec.distance >= self.desired_distance {
                if window_sec.start < window_sec.end {
                    window_sec.start += 1;
                } else {
                    window_sec.end += 1;
                }
            }
        }
        // after the while loop is finished, check that found fastest_section is valid and return
        if target_sec.target_value == 0.0 || target_sec.start == target_sec.end {
            Err(errors::InputDataError::NoSectionFound)
        } else {
            Ok(target_sec)
        }
    }
}

pub fn distance_in_bounds(
    window_distance: f64,
    desired_distance: f64,
    percentage_threshold: f64,
) -> bool {
    if (window_distance <= (desired_distance * (1.0 + percentage_threshold)))
        && (window_distance >= (desired_distance * (1.0 - percentage_threshold)))
    {
        true
    } else {
        false
    }
}

fn _generic_data_checks(
    coordinates_len: usize,
    times_len: usize,
) -> Result<(), errors::InputDataError> {
    if coordinates_len < 2 {
        return Err(errors::InputDataError::TooFewDataPoints);
    } else if coordinates_len != times_len {
        return Err(errors::InputDataError::InconsistentLength);
    } else {
        return Ok(());
    }
}

#[cfg(test)]
mod test_gem_finder {
    use super::*;

    #[test]
    fn test_finder_initialization() {
        let finder = InputData::new(
            10_000.,
            vec![(48.0, 8.0), (48.0, 8.1)],
            vec![123.4, 124.6],
            None,
            Some(0.01),
        )
        .unwrap();
        assert_eq!(finder.desired_distance, 10_000.);
        assert_eq!(finder.coordinates, vec!((48.0, 8.0), (48.0, 8.1)));
        assert_eq!(finder.times.values, vec!(123.4, 124.6));
    }

    #[test]
    fn test_compute_vector_of_distances() {
        let mut finder = InputData::new(
            10_000.,
            vec![(48.0, 8.0), (48.0, 8.1)],
            vec![123.4, 124.6],
            None,
            Some(0.01),
        )
        .unwrap();

        finder._compute_vector_of_distances();
        assert_eq!(finder.distances.values, vec!(0.0, 7448.684105664539));
    }
}

#[cfg(test)]
mod test_checks {
    use super::*;

    #[test]
    fn test_generic_data_checks_too_few_data_points() {
        // generate data with only one data point and assert that the TooFewDataPoints error is returned
        assert_eq!(
            _generic_data_checks(1, 1),
            Err(errors::InputDataError::TooFewDataPoints)
        );
    }

    #[test]
    fn test_generic_data_checks_inconsistent_length() {
        // coordinates and times vector have different lengths and assert that the InconsistentLength error is raised
        assert_eq!(
            _generic_data_checks(34, 37),
            Err(errors::InputDataError::InconsistentLength)
        );
    }

    #[test]
    fn test_check_if_total_distance_suffice() {
        // generate data where the overall distance is smaller than the desired
        // distance and assert that the DistanceTooSmall error is returned
        let mut finder = InputData::new(
            10_000.,
            vec![(48.123, 9.380), (48.123, 9.381)],
            vec![1608228940.8, 1608228941.8],
            None,
            Some(0.01),
        )
        .unwrap();
        finder._compute_vector_of_distances();
        assert_eq!(
            finder._check_if_total_distance_suffice(),
            Err(errors::InputDataError::DistanceTooSmall)
        );
    }
}
