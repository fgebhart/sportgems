use crate::dtypes;
use crate::gem_finder;
use crate::math;

fn _get_climb(
    section: &dtypes::WindowSection,
    altitudes: &dtypes::Altitudes,
    times: &dtypes::Times,
) -> f64 {
    let gained_altitude: f64 =
        altitudes.values[section.end as usize] - altitudes.values[section.start as usize];
    let duration = times.values[section.end as usize] - times.values[section.start as usize];
    math::_climb_equation(&gained_altitude, &duration)
}

pub fn update_sections_max_climb(
    input_data: &gem_finder::InputData,
    times: &dtypes::Times,
    window_sec: &mut dtypes::WindowSection,
    max_climb_sec: &mut dtypes::TargetSection,
) {
    window_sec.distance = input_data.distances.values[window_sec.end as usize]
        - input_data.distances.values[window_sec.start as usize];
    window_sec.climb = _get_climb(&window_sec, &input_data.altitudes, &times);
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
