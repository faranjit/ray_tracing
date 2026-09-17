pub struct Interval {
    min: f64,
    max: f64,
}

pub const INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.9999,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }
}
