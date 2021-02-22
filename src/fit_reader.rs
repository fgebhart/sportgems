extern crate fit;

use fit::Fit;
use std::path::PathBuf;

pub struct FitData {
    pub times: Vec<f32>,
    pub coordinates: Vec<(f32, f32)>,
}

pub fn parse_fit(path_to_fit: &str) -> FitData {
    let filepath = PathBuf::from(path_to_fit);
    let fit_file = Fit::new(&filepath);
    let mut payload = FitData { times: vec![], coordinates: vec![] };
    let mut time: f32;
    let mut lat: f32;
    let mut lon: f32;
    for message in fit_file {
        // println!("------------------");
        // println!("{:?}", m.values);
        lat = f32::NAN;
        lon = f32::NAN;
        time = f32::NAN;
        for record in message.values {
            // get latitude
            if record.field_num == 0 {
                match record.value {
                    fit::Value::F32(val) => {
                        lat = val;
                    }
                    _ => {}
                }
            }
            // get longitude
            if record.field_num == 1 {
                match record.value {
                    fit::Value::F32(val) => {
                        lon = val;
                    }
                    _ => {}
                }
            }
            // get timestamp
            if record.field_num == 253 {
                match record.value {
                    fit::Value::Time(val) => {
                        time = val as f32;
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
        assert_eq!(fit.coordinates[100], (49.4063, 8.695788));
        assert_eq!(fit.times[100], (1568474900.0));
    }
}
