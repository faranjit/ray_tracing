use std::ops;

use crate::rtweekend::Real;

#[derive(Clone, Copy)]
pub struct Interval {
    min: Real,
    max: Real,
}

pub const EMPTY: Interval = Interval {
    min: Real::INFINITY,
    max: Real::NEG_INFINITY,
};

pub const UNIVERSE: Interval = Interval {
    min: Real::NEG_INFINITY,
    max: Real::INFINITY,
};

pub const INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.9999,
};

impl Interval {
    pub fn new(min: Real, max: Real) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: Self, b: Self) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };

        Self { min, max }
    }

    pub fn min(&self) -> Real {
        self.min
    }

    pub fn max(&self) -> Real {
        self.max
    }

    pub fn surrounds(&self, x: Real) -> bool {
        self.min < x && x < self.max
    }

    pub fn contains(&self, x: Real) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn clamp(&self, x: Real) -> Real {
        x.clamp(self.min, self.max)
    }

    pub fn size(&self) -> Real {
        self.max - self.min
    }

    pub fn expand(&self, delta: Real) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl ops::Add<Real> for Interval {
    type Output = Interval;

    fn add(self, rhs: Real) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl ops::Add<Interval> for Real {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}
