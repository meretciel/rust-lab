
pub fn assert_eq_f64(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() < tolerance);
}

pub fn assert_eq_vec_f64(actual: Vec<f64>, expected: Vec<f64>, tolerance: f64) {
    assert_eq!(actual.len(), expected.len());
    for i in 0..actual.len() {
        assert_eq_f64(actual[i], expected[i], tolerance);
    }
}