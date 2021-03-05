use crate::dtypes;

pub const PI: f64 = 3.14159265358979323846264338327950288f64;

pub fn to_rad(degree: f64) -> f64 {
    degree / 180.0 * PI
}

pub fn calculate_distance(coordinate1: dtypes::Coordinate, coordinate2: dtypes::Coordinate) -> f64 {
    if coordinate1.lat == coordinate2.lat && coordinate1.lon == coordinate2.lon {
        return 0.0;
    } else {
        let mut distance = (to_rad(coordinate1.lat).sin() * to_rad(coordinate2.lat).sin()
            + to_rad(coordinate1.lat).cos()
                * to_rad(coordinate2.lat).cos()
                * (to_rad(coordinate1.lon - coordinate2.lon)).cos())
        .acos();
        // ensure distance is not null
        if distance.is_nan() {
            distance = 0.0;
        }
        // multiply by earth radius (nominal "zero tide" equatorial) in centimeter
        return distance * 6378100.0;
    }
}

pub fn _velocity_equation(distance: &f64, time: &f64) -> f64 {
    let velocity: f64 = distance / time;
    if !velocity.is_normal() {
        0.0
    } else {
        velocity
    }
}

pub fn _climb_equation(altitude: &f64, time: &f64) -> f64 {
    let climb: f64 = altitude / time;
    if !climb.is_normal() {
        0.0
    } else {
        climb
    }
}

fn _get_average(vec: &Vec<f64>) -> f64 {
    vec.iter().sum::<f64>() as f64 / vec.len() as f64
}

pub fn remove_outlier(input_vector: &Vec<f64>, percentage_threshold: f64) -> Vec<f64> {
    let avg: f64 = _get_average(&input_vector);
    let mut output_vector: Vec<f64> = vec![];
    for element in input_vector {
        if element > &(avg * (1. + percentage_threshold)) {
            output_vector.push(f64::NAN);
        } else if element < &(avg * (1. - percentage_threshold)) {
            output_vector.push(f64::NAN);
        } else {
            output_vector.push(*element)
        }
    }
    return output_vector;
}

#[cfg(test)]
mod test_calculate_distance {
    use super::*;

    #[test]
    fn test_to_rad() {
        assert_eq!(to_rad(180.0), PI);
        assert_eq!(to_rad(360.0), 2.0 * PI);
    }

    #[test]
    fn test_different_inputs() {
        // calculate distance of the same coordinates
        let coordinate_a: dtypes::Coordinate = dtypes::Coordinate {
            lat: 48.123,
            lon: 9.456,
        };
        let coordinate_b: dtypes::Coordinate = dtypes::Coordinate {
            lat: 48.123,
            lon: 9.456,
        };
        assert_eq!(calculate_distance(coordinate_a, coordinate_b), 0.0);

        // calculate distance of two arbitrary coordinates
        let coordinate_a: dtypes::Coordinate = dtypes::Coordinate {
            lat: 48.123,
            lon: 9.456,
        };
        let coordinate_b: dtypes::Coordinate = dtypes::Coordinate {
            lat: 49.678,
            lon: 9.567,
        };
        assert_eq!(
            calculate_distance(coordinate_a, coordinate_b),
            173291.21920642233
        );

        // only longitude is differing
        let coordinate_a: dtypes::Coordinate = dtypes::Coordinate {
            lat: 48.0,
            lon: 8.0,
        };
        let coordinate_b: dtypes::Coordinate = dtypes::Coordinate {
            lat: 48.0,
            lon: 8.1,
        };
        assert_eq!(
            calculate_distance(coordinate_a, coordinate_b),
            7448.684105664539
        );

        // only latitude is differing
        let coordinate_a: dtypes::Coordinate = dtypes::Coordinate {
            lat: 48.0,
            lon: 8.0,
        };
        let coordinate_b: dtypes::Coordinate = dtypes::Coordinate {
            lat: 48.1,
            lon: 8.0,
        };
        assert_eq!(
            calculate_distance(coordinate_a, coordinate_b),
            11131.884502572964
        );
    }

    #[test]
    fn test_edge_case() {
        let coordinate_a: dtypes::Coordinate = dtypes::Coordinate {
            lat: 49.09024318680168,
            lon: 7.9677597898989925,
        };
        let coordinate_b: dtypes::Coordinate = dtypes::Coordinate {
            lat: 49.09024335443974,
            lon: 7.967759286984802,
        };
        assert_eq!(calculate_distance(coordinate_a, coordinate_b), 0.0);
    }
}

#[cfg(test)]
mod test_optimized_target_value_formulas {
    use super::*;

    #[test]
    fn test_velocity_equation() {
        assert_eq!(_velocity_equation(&16.0, &2.0), 8.0);
        assert_eq!(_velocity_equation(&5.0, &1.0), 5.0);
        // division by zero should return zero
        assert_eq!(_velocity_equation(&3.0, &0.0), 0.0);
        // in case either of the inputs is NAN we expect also 0.0
        assert_eq!(_velocity_equation(&f64::NAN, &7.0), 0.0);
        assert_eq!(_velocity_equation(&4.0, &f64::NAN), 0.0);
        assert_eq!(_velocity_equation(&f64::NAN, &f64::NAN), 0.0)
    }

    #[test]
    fn test_climb_equation() {
        assert_eq!(_climb_equation(&16.0, &2.0), 8.0);
        assert_eq!(_climb_equation(&5.0, &1.0), 5.0);
        // division by zero should return zero
        assert_eq!(_climb_equation(&3.0, &0.0), 0.0);
        // in case either of the inputs is NAN we expect also 0.0
        assert_eq!(_climb_equation(&f64::NAN, &7.0), 0.0);
        assert_eq!(_climb_equation(&4.0, &f64::NAN), 0.0);
        assert_eq!(_climb_equation(&f64::NAN, &f64::NAN), 0.0)
    }
}


#[cfg(test)]
mod test_remove_outlier {
    use super::*;

    #[test]
    fn test_get_average() {
        assert_eq!(_get_average(&vec![1., 2., 3.]), 2.);
        assert_eq!(_get_average(&vec![-10., 20., 20.]), 10.);
    }

    // helper functions to compare two vectors containing f64::NAN to be equal
    fn eq_with_nan_eq(a: f64, b: f64) -> bool {
        (a.is_nan() && b.is_nan()) || (a == b)
    }
    
    fn vec_compare(va: &[f64], vb: &[f64]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
         va.iter()
           .zip(vb)
           .all(|(a,b)| eq_with_nan_eq(*a,*b))
    }

    #[test]
    fn test_remove_outlier_low_threshold() {
        let result_vec = remove_outlier(&vec![1., 1., 1., 1., 5., 1., 1., 1., 1., 1.], 0.50);   // 50% threshold
        // value of 5.0 is replace by NAN
        let expected_vec = vec![1., 1., 1., 1., f64::NAN, 1., 1., 1., 1., 1.];
        assert_eq!(vec_compare(&expected_vec, &result_vec), true)
    }

    #[test]
    fn test_remove_outlier_threshold_too_large() {
        let result_vec = remove_outlier(&vec![1., 1., 1., 1., 5., 1., 1., 1., 1., 1.], 3.00);   // 300% threshold
        // value of 5.0 is not replaced
        let expected_vec = vec![1., 1., 1., 1., 5., 1., 1., 1., 1., 1.];
        assert_eq!(vec_compare(&expected_vec, &result_vec), true)
    }
}