use fastrand_contrib::fastrand;

pub type Real = f32;

pub const INFINITY: Real = f32::INFINITY;

pub const PI: Real = 3.1415926535897932385;

#[inline]
pub const fn degrees_to_radians(degrees: Real) -> Real {
    degrees * PI / 180.0
}

#[inline]
pub fn random_double() -> Real {
    fastrand::f32()
}

#[inline]
pub fn random_double_range(min: Real, max: Real) -> Real {
    min + (max - min) * random_double()
}

#[inline]
pub fn random_int_range(min: usize, max: usize) -> usize {
    fastrand::usize(min..=max)
}
