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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_rad() {
        assert_eq!(to_rad(180.0), PI);
        assert_eq!(to_rad(360.0), 2.0 * PI);
    }

    #[test]
    fn test_calculate_distance() {
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
