use crate::dtypes;
use crate::gem_finder;
use crate::math;

fn _get_velocity(
    section: &dtypes::WindowSection,
    distances: &dtypes::Distances,
    times: &dtypes::Times,
) -> f64 {
    let gained_distance =
        distances.values[section.end as usize] - distances.values[section.start as usize];
    let duration = times.values[section.end as usize] - times.values[section.start as usize];
    math::_velocity_formula(&gained_distance, &duration)
}

pub fn _update_sections_max_velocity(
    input_data: &gem_finder::InputData,
    times: &dtypes::Times,
    window_sec: &mut dtypes::WindowSection,
    fastest_sec: &mut dtypes::TargetSection,
) {
    window_sec.velocity = _get_velocity(&window_sec, &input_data.distances, &times);
    // update fastest_sec only in case the current distance
    // is not larger than the required distance + 1%
    if window_sec.distance <= (input_data.desired_distance as f64) * 1.01 {
        if window_sec.velocity > fastest_sec.target_value {
            fastest_sec.start = window_sec.start;
            fastest_sec.end = window_sec.end;
            fastest_sec.target_value = window_sec.velocity;
        }
    }
}
