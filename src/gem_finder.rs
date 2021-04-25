use crate::exc;
use crate::math;
use crate::{dtypes, exc::InputDataError};

pub const DEFAULT_TOLERANCE: f64 = 0.01;

#[derive(Debug, PartialEq)]
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
    ) -> Result<InputData, exc::InputDataError> {
        if desired_distance <= 0.0 {
            Err(InputDataError::InvalidDesiredDistance)
        } else {
            match generic_data_checks(&coordinates, &times) {
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
    }

    pub fn check_if_total_distance_suffice(&self) -> Result<(), exc::InputDataError> {
        let total_distance = self.distances.values.last().unwrap().clone();
        if self.desired_distance > total_distance {
            return Err(exc::InputDataError::DistanceTooSmall);
        } else {
            return Ok(());
        }
    }

    pub fn compute_vector_of_distances(&mut self) {
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
    pub fn search_section(
        &mut self,
        update_func: fn(&InputData, &mut dtypes::WindowSection, &mut dtypes::TargetSection),
    ) -> Result<dtypes::TargetSection, exc::InputDataError> {
        let mut window_sec = dtypes::WindowSection::default();
        let mut target_sec = dtypes::TargetSection::default();
        while window_sec.end < self.distances.values.len() as u32 - 1 {
            // println!("{:?}", window_sec);

            if window_sec.distance < self.desired_distance {
                // build up section to get closer to the desired length of desired_distance
                window_sec.end += 1;
            } else {
                // now move the start index further, but ensure that start index does not overtake end index
                if window_sec.start < window_sec.end {
                    window_sec.start += 1;
                } else {
                    window_sec.end += 1;
                }
            }
            update_func(&self, &mut window_sec, &mut target_sec);
        }
        // after the while loop is finished, check that found fastest_section is valid and return
        if target_sec.target_value == 0.0 || target_sec.start == target_sec.end {
            Err(exc::InputDataError::NoSectionFound)
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

pub fn get_distance(distances: &Vec<f64>, start: usize, end: usize) -> f64 {
    distances[end] - distances[start + 1]
}

fn generic_data_checks(
    coordinates: &Vec<(f64, f64)>,
    times: &Vec<f64>,
) -> Result<(), exc::InputDataError> {
    if coordinates.len() != times.len() {
        return Err(exc::InputDataError::InconsistentLength);
    }
    let mut coordinates_normal = coordinates.clone();
    let mut times_normal = times.clone();
    coordinates_normal.retain(|&i| (i.0.is_normal() && i.1.is_normal()));
    times_normal.retain(|&i| i.is_normal());
    if coordinates_normal.len() < 2 || times_normal.len() < 2 {
        return Err(exc::InputDataError::TooFewDataPoints);
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
    fn test_invalid_desired_distance_zero() {
        let finder = InputData::new(
            0.,
            vec![(48.0, 8.0), (48.0, 8.1)],
            vec![123.4, 124.6],
            None,
            Some(0.01),
        );
        assert_eq!(finder, Err(exc::InputDataError::InvalidDesiredDistance));
    }

    #[test]
    fn test_invalid_desired_distance_negative() {
        let finder = InputData::new(
            -10.,
            vec![(48.0, 8.0), (48.0, 8.1)],
            vec![123.4, 124.6],
            None,
            Some(0.01),
        );
        assert_eq!(finder, Err(exc::InputDataError::InvalidDesiredDistance));
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

        finder.compute_vector_of_distances();
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
            generic_data_checks(&vec![(1., 1.)], &vec![1.]),
            Err(exc::InputDataError::TooFewDataPoints)
        );
    }

    #[test]
    fn test_generic_data_checks_inconsistent_length() {
        // coordinates and times vector have different lengths and assert that the InconsistentLength error is raised
        assert_eq!(
            generic_data_checks(&vec![(1., 1.), (2., 2.)], &vec![1., 2., 3.]),
            Err(exc::InputDataError::InconsistentLength)
        );
    }

    #[test]
    fn test_generic_data_checks_coordinates_are_nan() {
        // input coordinates consist of nans only, TooFewDataPoints should be raised
        assert_eq!(
            generic_data_checks(
                &vec![(f64::NAN, f64::NAN), (f64::NAN, f64::NAN)],
                &vec![1., 2.]
            ),
            Err(exc::InputDataError::TooFewDataPoints)
        );
    }

    #[test]
    fn test_generic_data_checks_times_are_nan() {
        // input times consist of nans only, TooFewDataPoints should be raised
        assert_eq!(
            generic_data_checks(&vec![(1., 1.), (2., 2.)], &vec![f64::NAN, f64::NAN]),
            Err(exc::InputDataError::TooFewDataPoints)
        );
    }

    #[test]
    fn test_generic_data_checks_one_data_point_in_times_is_normal() {
        // input times consist of one normal element only, TooFewDataPoints should be raised
        assert_eq!(
            generic_data_checks(&vec![(1., 1.), (2., 2.)], &vec![1., f64::NAN]),
            Err(exc::InputDataError::TooFewDataPoints)
        );
    }

    #[test]
    fn test_generic_data_checks_two_data_points_in_times_are_normal() {
        // input times consist of two normal element, result should be ok
        assert_eq!(
            generic_data_checks(&vec![(1., 1.), (2., 2.), (3., 3.)], &vec![1., 2., f64::NAN]),
            Ok(())
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
        finder.compute_vector_of_distances();
        assert_eq!(
            finder.check_if_total_distance_suffice(),
            Err(exc::InputDataError::DistanceTooSmall)
        );
    }
}
