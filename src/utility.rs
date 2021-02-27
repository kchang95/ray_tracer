const F64_CMP_THRESHOLD: f64 = 0.00001;

pub fn equal_float(a: &f64, b: &f64) -> bool {
    let difference = a - b;
    if difference.abs() < F64_CMP_THRESHOLD {
        true
    } else {
        false
    }
}
