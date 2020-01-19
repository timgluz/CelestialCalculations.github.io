pub const DISTANCE_TOLERANCE: f64 = 0.1e-5;

pub fn is_close(expected_val: f64, real_val: f64) -> bool {
    (expected_val - real_val).abs() <= DISTANCE_TOLERANCE
}

pub fn assert_close(expected_val: f64, real_val: f64) {
    assert!(
        is_close(expected_val, real_val),
        "value {} differs from {} more than {}",
        real_val,
        expected_val,
        DISTANCE_TOLERANCE
    )
}
