extern crate fit;

use fit::Fit;
use std::path::PathBuf;
// use enum_map::{enum_map, Enum, EnumMap};

pub struct FitData {
    pub times: Vec<f64>,
    pub coordinates: Vec<(f64, f64)>,
    pub calories: u16,
}


fn _match_one_time_values(record: &fit::DataField, fit_data: &mut FitData) {
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

fn _match_time_series_values(record: &fit::DataField, latitude: &mut f64, longitude: &mut f64, timestamp: &mut f64) {
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
}

pub fn parse_fit(path_to_fit: &str) -> FitData {
    let filepath = PathBuf::from(path_to_fit);
    let fit_file = Fit::new(&filepath);
    let mut fit_data = FitData {
        times: vec![],
        coordinates: vec![],
        calories: 0,
    };
    let mut timestamp: f64;
    let mut latitude: f64;
    let mut longitude: f64;
    for message in fit_file {
        println!("------------------");
        println!("{:?}", message.values);
        latitude = f64::NAN;
        longitude = f64::NAN;
        timestamp = f64::NAN;
        for record in message.values {
            // get time series values
            _match_time_series_values(&record, &mut latitude, &mut longitude, &mut timestamp);
            // get one time values
            _match_one_time_values(&record, &mut fit_data);
        }
        fit_data.times.push(timestamp);
        fit_data.coordinates.push((latitude, longitude));
    }
    // println!("times: {:?}", payload.times);
    // println!("coordinates: {:?}", payload.coordinates);
    // println!("len times: {:?}", payload.times.len());
    // println!("len coordinates: {:?}", payload.coordinates.len());
    assert_eq!(fit_data.times.len(), fit_data.coordinates.len());
    return fit_data;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_fit() {
        let fit = parse_fit("tests/data/2019-09-14-17-22-05.fit");
        assert_eq!(fit.times.len(), fit.coordinates.len());
        assert!(fit.times[0].is_nan());
        assert_eq!(fit.coordinates[100], (49.40629959106445, 8.695788383483887));
        assert_eq!(fit.times[100], (1568474841.0));
        assert_eq!(fit.calories, 432);
    }
}
