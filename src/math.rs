pub fn _velocity_formula(distance: &f64, time: &f64) -> f64 {
    let velocity: f64 = distance / time;
    if !velocity.is_normal() {
        0.0
    } else {
        velocity
    }
}

pub fn _climb_formula(altitude: &f64, time: &f64) -> f64 {
    let climb: f64 = altitude / time;
    if !climb.is_normal() {
        0.0
    } else {
        climb
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_velocity_formula() {
        assert_eq!(_velocity_formula(&16.0, &2.0), 8.0);
        assert_eq!(_velocity_formula(&5.0, &1.0), 5.0);
        // division by zero should return zero
        assert_eq!(_velocity_formula(&3.0, &0.0), 0.0);
        // in case either of the inputs is NAN we expect also 0.0
        assert_eq!(_velocity_formula(&f64::NAN, &7.0), 0.0);
        assert_eq!(_velocity_formula(&4.0, &f64::NAN), 0.0);
        assert_eq!(_velocity_formula(&f64::NAN, &f64::NAN), 0.0)
    }

    #[test]
    fn test_climb_formula() {
        assert_eq!(_climb_formula(&16.0, &2.0), 8.0);
        assert_eq!(_climb_formula(&5.0, &1.0), 5.0);
        // division by zero should return zero
        assert_eq!(_climb_formula(&3.0, &0.0), 0.0);
        // in case either of the inputs is NAN we expect also 0.0
        assert_eq!(_climb_formula(&f64::NAN, &7.0), 0.0);
        assert_eq!(_climb_formula(&4.0, &f64::NAN), 0.0);
        assert_eq!(_climb_formula(&f64::NAN, &f64::NAN), 0.0)
    }
}
