pub const INFINITY: f64 = f64::INFINITY;

pub const PI: f64 = 3.1415926535897932385;

#[inline]
pub const fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline(always)]
pub fn random_double() -> f64 {
    rand::random_range(0.0..1.0)
}

#[inline(always)]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
