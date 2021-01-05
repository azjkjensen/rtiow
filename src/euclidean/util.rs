use rand::random;

#[inline(always)]
pub fn degrees_to_rads(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

#[inline(always)]
pub fn random_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random::<f64>()
}

#[inline(always)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
