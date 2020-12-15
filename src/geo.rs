
pub const PI: f64 = 3.14159265358979323846264338327950288f64;



pub fn to_rad(degree: f64) -> f64 {
    degree/180.0 * PI
}

pub fn calculate_distance() -> f64 {
    let distance: f64 = 1000.0;
    distance
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_rad() {
        assert_eq!(4, 4);
        assert_eq!(PI, to_rad(180.0));
        assert_eq!(2.0*PI, to_rad(360.0));
    }
}