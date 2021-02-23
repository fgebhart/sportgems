extern crate fit;

use fit::Fit;
use std::path::PathBuf;

pub struct FitData {
    pub times: Vec<f64>,
    pub coordinates: Vec<(f64, f64)>,
}

pub fn parse_fit(path_to_fit: &str) -> FitData {
    let filepath = PathBuf::from(path_to_fit);
    let fit_file = Fit::new(&filepath);
    let mut payload = FitData {
        times: vec![],
        coordinates: vec![],
    };
    let mut time: f64;
    let mut lat: f64;
    let mut lon: f64;
    for message in fit_file {
        // println!("------------------");
        // println!("{:?}", m.values);
        lat = f64::NAN;
        lon = f64::NAN;
        time = f64::NAN;
        for record in message.values {
            // get latitude
            if record.field_num == 0 {
                match record.value {
                    fit::Value::F32(val) => {
                        lat = val as f64;
                    }
                    _ => {}
                }
            }
            // get longitude
            if record.field_num == 1 {
                match record.value {
                    fit::Value::F32(val) => {
                        lon = val as f64;
                    }
                    _ => {}
                }
            }
            // get timestamp
            if record.field_num == 253 {
                match record.value {
                    fit::Value::Time(val) => {
                        time = val as f64;
                    }
                    _ => {}
                }
            }
        }
        payload.times.push(time);
        payload.coordinates.push((lat, lon));
    }
    println!("times: {:?}", payload.times);
    println!("coordinates: {:?}", payload.coordinates);
    println!("len times: {:?}", payload.times.len());
    println!("len coordinates: {:?}", payload.coordinates.len());
    assert_eq!(payload.times.len(), payload.coordinates.len());
    return payload;
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
    }
}
