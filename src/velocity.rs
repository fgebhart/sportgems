use crate::dtypes;
use crate::errors;
use crate::fit_reader;
use crate::gem_finder;
use crate::math;

fn _get_velocity(
    section: &dtypes::WindowSection,
    gained_distance: &f64,
    times: &dtypes::Times,
) -> f64 {
    let duration = times.values[section.end as usize] - times.values[section.start as usize];
    math::_velocity_equation(&gained_distance, &duration)
}

pub fn _update_sections_max_velocity(
    input_data: &gem_finder::InputData,
    window_sec: &mut dtypes::WindowSection,
    fastest_sec: &mut dtypes::TargetSection,
) {
    window_sec.distance = input_data.distances.values[window_sec.end as usize]
        - input_data.distances.values[window_sec.start as usize];
    window_sec.velocity = _get_velocity(&window_sec, &window_sec.distance, &input_data.times);
    // update fastest_sec only in case the current distance is equal to the desired distance +- 1% and velocity is larger
    if gem_finder::distance_in_bounds(
        window_sec.distance,
        input_data.desired_distance,
        input_data.tolerance,
    ) && window_sec.velocity > fastest_sec.target_value
    {
        fastest_sec.start = window_sec.start;
        fastest_sec.end = window_sec.end;
        fastest_sec.target_value = window_sec.velocity;
    }
}

pub fn find_fastest_section(
    desired_distance: f64,
    coordinates: Vec<(f64, f64)>,
    times: Vec<f64>,
    tolerance: Option<f64>,
) -> Result<dtypes::TargetSection, errors::InputDataError> {
    match gem_finder::InputData::new(desired_distance, coordinates, times, None, tolerance) {
        Err(e) => Err(e),
        Ok(mut finder) => {
            finder._compute_vector_of_distances();
            match finder._check_if_total_distance_suffice() {
                Ok(_) => return Ok(finder._search_section(_update_sections_max_velocity)),
                Err(e) => return Err(e),
            }
        }
    }
}

pub fn find_fastest_section_in_fit(
    desired_distance: f64,
    path_to_fit: &str,
    tolerance: Option<f64>,
) -> Result<dtypes::TargetSection, errors::InputDataError> {
    let fit_data: fit_reader::FitData = fit_reader::parse_fit(path_to_fit);
    match find_fastest_section(
        desired_distance,
        fit_data.coordinates,
        fit_data.times,
        tolerance,
    ) {
        Err(e) => Err(e),
        Ok(result) => Ok(result),
    }
}

#[cfg(test)]
mod test_find_fastest_section {
    use super::*;
    #[test]
    fn test_find_fastest_section_edge_case_no_change_in_time() {
        // test case where coordinates are changing but time does not, this would lead to infinite velocity
        let desired_distance = 1_000.;
        let coordinates = vec![(48.0, 8.0), (48.0, 8.1)];
        let times = vec![123.4, 123.4];

        // in this scenario we expect no valid section to be found
        let fastest_section =
            find_fastest_section(desired_distance, coordinates, times, Some(0.01)).unwrap();
        assert_eq!(fastest_section, dtypes::TargetSection::default());
    }

    #[test]
    fn test_find_fastest_section_dummy_values() {
        // add test with dummy values
        let desired_distance = 1_000.;
        let coordinates = vec![
            (48.123, 9.35),
            (48.123, 9.36),
            (48.123, 9.37),
            (48.123, 9.38),
        ];
        let times = vec![1608228953.8, 1608228954.8, 1608228955.8, 1608228956.8];

        // in this scenario we expect a valid result section (use 50% tolerance in order to be able to match section
        // in this low resolution activity data)
        let fastest_section =
            find_fastest_section(desired_distance, coordinates, times, Some(0.5)).unwrap();
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 0);
        assert_eq!(fastest_section.end, 1);
        assert_eq!(fastest_section.target_value.round(), 743.0);
    }
    #[test]
    fn test_find_fastest_section_nan_values() {
        // add test with null values
        let desired_distance = 1_000.;
        let coordinates = vec![
            (f64::NAN, f64::NAN),
            (48.123, 9.36),
            (48.123, 9.37),
            (48.123, 9.38),
        ];
        let times = vec![1608228940.8, 1608228950.8, 1608228960.8, 1608228970.8];

        // in this scenario we expect a valid result section
        let fastest_section =
            find_fastest_section(desired_distance, coordinates, times, Some(0.5)).unwrap();
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 0);
        assert_eq!(fastest_section.end, 3);
        assert_eq!(fastest_section.target_value.round(), 50.0);
    }

    #[test]
    fn test_find_fastest_section_larger_data() {
        // add test with more values
        let desired_distance = 250.;
        let coordinates = vec![
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
        ];
        let times = vec![
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
        ];

        // in this scenario we expect a valid result section
        let fastest_section =
            find_fastest_section(desired_distance, coordinates, times, Some(10.0)).unwrap();
        assert_eq!(fastest_section.valid, true);
        assert_eq!(fastest_section.start, 2); // at index 2 the step distance increases
        assert_eq!(fastest_section.end, 5);
        assert_eq!(fastest_section.target_value.round(), 7.0);
    }

    pub const FIT_FILE: &str = "tests/data/2019-09-14-17-22-05.fit";

    #[test]
    fn test_find_fastest_section_in_fit_one_km() {
        let result = find_fastest_section_in_fit(1_000., FIT_FILE, Some(0.01)).unwrap();
        assert_eq!(result.valid, true);
        assert_eq!(result.start, 635);
        assert_eq!(result.end, 725);
        assert_eq!(result.target_value.round(), 3.0);
    }

    #[test]
    fn test_find_fastest_section_in_fit_two_km() {
        let result = find_fastest_section_in_fit(2_000., FIT_FILE, Some(0.01)).unwrap();
        assert_eq!(result.valid, true);
        assert_eq!(result.start, 543);
        assert_eq!(result.end, 821);
        assert_eq!(result.target_value.round(), 2.0);
    }

    #[test]
    fn test_find_fastest_section_in_fit_three_km() {
        let result = find_fastest_section_in_fit(3_000., FIT_FILE, Some(0.01)).unwrap();
        assert_eq!(result.valid, true);
        assert_eq!(result.start, 434);
        assert_eq!(result.end, 945);
        assert_eq!(result.target_value.round(), 2.0);
    }

    #[test]
    fn test_find_fastest_section_in_fit_four_km() {
        let result = find_fastest_section_in_fit(4_000., FIT_FILE, Some(0.01)).unwrap();
        assert_eq!(result.valid, true);
        assert_eq!(result.start, 300);
        assert_eq!(result.end, 1095);
        assert_eq!(result.target_value.round(), 2.0);
    }
}
