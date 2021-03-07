use crate::dtypes;
use crate::errors;
use crate::fit_reader;
use crate::gem_finder;
use crate::math;

fn _get_climb(
    section: &dtypes::WindowSection,
    altitudes: &dtypes::Altitudes,
    times: &dtypes::Times,
) -> f64 {
    let gained_altitude_in_section: f64 = _get_gained_altitude_in_section(
        &altitudes.values,
        section.start as usize,
        section.end as usize,
    );
    let duration = times.values[section.end as usize] - times.values[section.start as usize];
    // println!("gained altitude: {:?}", gained_altitude_in_section);
    // println!("duration: {:?}", duration);
    math::_climb_equation(&gained_altitude_in_section, &(duration / 60.))
}

fn _get_gained_altitude_in_section(altitudes: &Vec<f64>, start: usize, end: usize) -> f64 {
    let mut section = altitudes[start..end].to_vec();
    // drop NAN values
    section.retain(|&i| i.is_normal());
    let mut altitude_increments: Vec<f64> = vec![];
    for i in 0..section.len() - 1 {
        altitude_increments.push(section[i + 1] - section[i]);
    }
    // only keep positive elements
    altitude_increments.retain(|&i| i > 0.);
    return altitude_increments.iter().sum();
}

pub fn update_sections_max_climb(
    input_data: &gem_finder::InputData,
    window_sec: &mut dtypes::WindowSection,
    max_climb_sec: &mut dtypes::TargetSection,
) {
    window_sec.distance = input_data.distances.values[window_sec.end as usize]
        - input_data.distances.values[window_sec.start as usize];
    window_sec.climb = _get_climb(&window_sec, &input_data.altitudes, &input_data.times);
    // update fastest_sec only in case the current distance is equal to the desired distance +- 1% and velocity is larger
    if gem_finder::distance_in_bounds(
        window_sec.distance,
        input_data.desired_distance,
        input_data.tolerance,
    ) && window_sec.climb > max_climb_sec.target_value
    {
        max_climb_sec.start = window_sec.start;
        max_climb_sec.end = window_sec.end;
        max_climb_sec.target_value = window_sec.climb;
    }
}

pub fn specific_data_check(
    input_data: &gem_finder::InputData,
) -> Result<(), errors::InputDataError> {
    if input_data.altitudes.values.len() < 2 {
        return Err(errors::InputDataError::TooFewDataPoints);
    } else if input_data.coordinates.len() != input_data.altitudes.values.len() {
        return Err(errors::InputDataError::InconsistentLength);
    } else {
        return Ok(());
    }
}

pub fn find_best_climb_section(
    desired_distance: f64,
    coordinates: Vec<(f64, f64)>,
    times: Vec<f64>,
    altitudes: Vec<f64>,
    tolerance: Option<f64>,
) -> Result<dtypes::TargetSection, errors::InputDataError> {
    match gem_finder::InputData::new(
        desired_distance,
        coordinates,
        times,
        Some(altitudes),
        tolerance,
    ) {
        Err(e) => Err(e),
        Ok(mut finder) => {
            finder._compute_vector_of_distances();
            match finder._check_if_total_distance_suffice() {
                Ok(_) => {
                    match finder._search_section(update_sections_max_climb) {
                        Ok(result) => return Ok(result),
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}

pub fn find_best_climb_section_in_fit(
    desired_distance: f64,
    path_to_fit: &str,
    tolerance: Option<f64>,
) -> Result<dtypes::TargetSection, errors::InputDataError> {
    let fit_data: fit_reader::FitData = fit_reader::parse_fit(path_to_fit);
    let filtered_altitudes = math::remove_outliers(&fit_data.altitudes, 10.0); // = 1000 %
    match find_best_climb_section(
        desired_distance,
        fit_data.coordinates,
        fit_data.times,
        filtered_altitudes,
        tolerance,
    ) {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test_climb {
    use super::*;

    #[test]
    fn test_get_gained_altitude_in_section_all_values() {
        let altitudes = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = _get_gained_altitude_in_section(&altitudes, 0, altitudes.len());
        let expected = 4.0;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_gained_altitude_in_section_slice_only() {
        let altitudes = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = _get_gained_altitude_in_section(&altitudes, 1, altitudes.len() - 1);
        let expected = 2.0;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_gained_altitude_in_section_also_going_downhill() {
        let altitudes = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0];
        let result = _get_gained_altitude_in_section(&altitudes, 0, altitudes.len());
        // expect 4.0, since only climbing uphill counts, negative values (going downhill) will be dropped
        let expected = 4.0;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_gained_altitude_in_section_including_nan() {
        let altitudes = vec![1.0, 2.0, 3.0, f64::NAN, 4.0];
        let result = _get_gained_altitude_in_section(&altitudes, 0, altitudes.len());
        // expect 3.0 since the nan value will be dropped
        let expected = 3.0;
        assert_eq!(expected, result);
    }

    pub const FIT_FILE: &str = "tests/data/2019-09-14-17-22-05.fit";

    #[test]
    fn test_find_best_climb_section_in_fit() {
        let result = find_best_climb_section_in_fit(1_000., FIT_FILE, Some(0.01)).unwrap();
        assert_eq!(result.start, 344);
        assert_eq!(result.end, 586);
        assert_eq!(result.target_value.round(), 6.0);
    }

    #[test]
    fn test_find_best_climb_section_in_fit_larger_section() {
        let result = find_best_climb_section_in_fit(3_000., FIT_FILE, Some(0.01)).unwrap();
        assert_eq!(result.start, 63);
        assert_eq!(result.end, 708);
        assert_eq!(result.target_value.round(), 4.0);
    }
}
