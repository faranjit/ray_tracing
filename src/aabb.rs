use std::ops;

use crate::{
    interval::{EMPTY, Interval, UNIVERSE},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub const fn empty() -> Self {
        Self {
            x: EMPTY,
            y: EMPTY,
            z: EMPTY,
        }
    }

    pub const fn universe() -> Self {
        Self {
            x: UNIVERSE,
            y: UNIVERSE,
            z: UNIVERSE,
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self::pad_to_minimums(x, y, z)
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a[0] < b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };

        let y = if a[1] < b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };

        let z = if a[2] < b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };

        Self::pad_to_minimums(x, y, z)
    }

    pub fn from_boxes(box0: &Self, box1: &Self) -> Self {
        let x = Interval::from_intervals(box0.x, box1.x);
        let y = Interval::from_intervals(box0.y, box1.y);
        let z = Interval::from_intervals(box0.z, box1.z);

        Self { x, y, z }
    }

    pub fn axis_interval(&self, axis: usize) -> Interval {
        match axis {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn longest_axis(&self) -> usize {
        // Returns the index of the longest axis of the bounding box.
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let orig = r.origin();
        let mut min = ray_t.min();
        let mut max = ray_t.max();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = r.inv_direction()[axis];
            let t0 = (ax.min() - orig[axis]) * adinv;
            let t1 = (ax.max() - orig[axis]) * adinv;

            min = min.max(t0.min(t1));
            max = max.min(t0.max(t1));

            if max <= min {
                return false;
            }
        }
        true
    }

    fn pad_to_minimums(x: Interval, y: Interval, z: Interval) -> Self {
        let delta = 0.0001;
        Self {
            x: if x.size() < delta { x.expand(delta) } else { x },
            y: if y.size() < delta { y.expand(delta) } else { y },
            z: if z.size() < delta { z.expand(delta) } else { z },
        }
    }
}

impl ops::Add<Vec3> for Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl ops::Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, rhs: Aabb) -> Self::Output {
        rhs + self
    }
}
