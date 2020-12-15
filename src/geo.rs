pub const PI: f64 = 3.14159265358979323846264338327950288f64;

pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}

pub struct Times {
    pub values: Vec<f64>,
}

pub struct Distances {
    pub values: Vec<f64>,
}

pub fn to_rad(degree: f64) -> f64 {
    degree / 180.0 * PI
}

pub fn calculate_distance(coordinate1: Coordinate, coordinate2: Coordinate) -> f64 {
    if coordinate1.lat == coordinate2.lat && coordinate1.lon == coordinate2.lon {
        return 0.0;
    } else {
        let distance: f64;
        distance = (to_rad(coordinate1.lat).sin() * to_rad(coordinate2.lat).sin()
            + to_rad(coordinate1.lat).cos()
                * to_rad(coordinate2.lat).cos()
                * (to_rad(coordinate1.lon - coordinate2.lon)).cos())
        .acos();
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
        let coordinate_a: Coordinate = Coordinate {
            lat: 48.123,
            lon: 9.456,
        };
        let coordinate_b: Coordinate = Coordinate {
            lat: 48.123,
            lon: 9.456,
        };
        assert_eq!(calculate_distance(coordinate_a, coordinate_b), 0.0);

        // calculate distance of two arbitrary coordinates
        let coordinate_a: Coordinate = Coordinate {
            lat: 48.123,
            lon: 9.456,
        };
        let coordinate_b: Coordinate = Coordinate {
            lat: 49.678,
            lon: 9.567,
        };
        assert_eq!(
            calculate_distance(coordinate_a, coordinate_b),
            173291.21920647446
        );

        // only longitude is differing
        let coordinate_a: Coordinate = Coordinate {
            lat: 48.0,
            lon: 8.0,
        };
        let coordinate_b: Coordinate = Coordinate {
            lat: 48.0,
            lon: 8.1,
        };
        assert_eq!(
            calculate_distance(coordinate_a, coordinate_b),
            7448.684105058203
        );

        // only latitude is differing
        let coordinate_a: Coordinate = Coordinate {
            lat: 48.0,
            lon: 8.0,
        };
        let coordinate_b: Coordinate = Coordinate {
            lat: 48.1,
            lon: 8.0,
        };
        assert_eq!(
            calculate_distance(coordinate_a, coordinate_b),
            11131.884502167246
        );
    }
}
