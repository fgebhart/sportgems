use crate::dtypes;
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
    // update max_climb_sec only in case the current distance
    // is not larger than the required distance + 1%
    if window_sec.distance <= (input_data.desired_distance as f64) * 1.01 {
        if window_sec.climb > max_climb_sec.target_value {
            max_climb_sec.start = window_sec.start;
            max_climb_sec.end = window_sec.end;
            max_climb_sec.target_value = window_sec.climb;
        }
    }
}

pub fn specific_data_check(
    input_data: &gem_finder::InputData,
) -> Result<(), gem_finder::InputDataError> {
    if input_data.altitudes.values.len() < 2 {
        return Err(gem_finder::InputDataError::TooFewDataPoints);
    } else if input_data.coordinates.len() != input_data.altitudes.values.len() {
        return Err(gem_finder::InputDataError::InconsistentLength);
    } else {
        return Ok(());
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
}
