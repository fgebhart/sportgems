extern crate fit;

use fit::Fit;
use std::path::PathBuf;

pub struct FitData {
    pub calories: u16,
    pub times: Vec<f64>,
    pub coordinates: Vec<(f64, f64)>,
    pub altitudes: Vec<f64>,
}

fn match_one_time_values(record: &fit::DataField, fit_data: &mut FitData) {
    // get calories
    if record.field_num == 11 {
        match record.value {
            fit::Value::U16(val) => {
                fit_data.calories = val;
            }
            _ => {}
        }
    }
}

fn match_time_series_values(
    record: &fit::DataField,
    latitude: &mut f64,
    longitude: &mut f64,
    timestamp: &mut f64,
    altitude: &mut f64,
) {
    // get latitude
    if record.field_num == 0 {
        match record.value {
            fit::Value::F32(val) => {
                *latitude = val as f64;
            }
            _ => {}
        }
    }
    // get longitude
    if record.field_num == 1 {
        match record.value {
            fit::Value::F32(val) => {
                *longitude = val as f64;
            }
            _ => {}
        }
    }
    // get timestamp
    if record.field_num == 253 {
        match record.value {
            fit::Value::Time(val) => {
                *timestamp = val as f64;
            }
            _ => {}
        }
    }
    // get altitude
    if record.field_num == 2 {
        match record.value {
            fit::Value::U16(val) => {
                *altitude = val as f64 / 10.0; // turn cm into meter
            }
            _ => {}
        }
    }
}

pub fn parse_fit(path_to_fit: &str) -> FitData {
    let filepath = PathBuf::from(path_to_fit);
    let fit_file = Fit::new(&filepath);
    let mut fit_data = FitData {
        times: vec![],
        coordinates: vec![],
        altitudes: vec![],
        calories: 0,
    };
    let mut timestamp: f64;
    let mut latitude: f64;
    let mut longitude: f64;
    let mut altitude: f64;
    for message in fit_file {
        // println!("------------------");
        // println!("{:?}", message.values);
        latitude = f64::NAN;
        longitude = f64::NAN;
        timestamp = f64::NAN;
        altitude = f64::NAN;
        for record in message.values {
            // get time series values, e.g. coordinates, timestamps and altitude
            match_time_series_values(
                &record,
                &mut latitude,
                &mut longitude,
                &mut timestamp,
                &mut altitude,
            );
            // get one time values, e.g. overall calories, ...
            match_one_time_values(&record, &mut fit_data);
        }
        fit_data.times.push(timestamp);
        fit_data.altitudes.push(altitude);
        fit_data.coordinates.push((latitude, longitude));
    }
    assert_eq!(fit_data.times.len(), fit_data.coordinates.len());
    assert_eq!(fit_data.times.len(), fit_data.altitudes.len());
    return fit_data;
}

#[cfg(test)]
mod test_fit_reader {
    use super::*;

    pub const FIT_FILE: &str = "tests/data/2019-09-14-17-22-05.fit";

    #[test]
    fn test_parse_fit() {
        let fit = parse_fit(FIT_FILE);
        assert_eq!(fit.times.len(), fit.coordinates.len());
        assert!(fit.times[0].is_nan());
        assert_eq!(fit.coordinates[100], (49.40629959106445, 8.695788383483887));
        assert_eq!(fit.times[100], (1568474841.0));
        assert_eq!(fit.altitudes[100], (254.9));
        assert_eq!(fit.calories, 432);
    }
}
