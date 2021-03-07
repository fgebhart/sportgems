use std::default::Default;

pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug)]
pub struct Times {
    pub values: Vec<f64>,
}

#[derive(Debug)]
pub struct Distances {
    pub values: Vec<f64>,
}

#[derive(Debug)]
pub struct Altitudes {
    pub values: Vec<f64>,
}

#[derive(PartialEq, Debug)]
pub struct TargetSection {
    pub start: u32,
    pub end: u32,
    pub target_value: f64, // to be maximized
}

impl Default for TargetSection {
    fn default() -> TargetSection {
        TargetSection {
            start: 0,
            end: 0,
            target_value: 0.0,
        }
    }
}

// window section used to scan through the activity
#[derive(Debug, Clone)]
pub struct WindowSection {
    pub start: u32,
    pub end: u32,
    pub distance: f64,
    pub velocity: f64,
    pub climb: f64,
}

impl Default for WindowSection {
    fn default() -> WindowSection {
        WindowSection {
            start: 0,
            end: 0,
            distance: 0.0,
            velocity: 0.0,
            climb: 0.0,
        }
    }
}
