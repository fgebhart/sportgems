pub const TEST_PRECISION: f64 = 0.0001;


pub fn assert_gem_eq(left: (bool, u32, u32, f64), right: (bool, u32, u32, f64), section: u32) {
    assert_eq!(left.0, right.0);
    assert_eq!(left.1, right.1);
    assert_eq!(left.2, right.2);
    let diff: f64 = left.3 - right.3;
    assert!(diff.abs() <= TEST_PRECISION, "testing fastest {}", section);
}
