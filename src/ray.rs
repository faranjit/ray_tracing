use crate::{rtweekend::Real, vec3::{Point3, Vec3}};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    inv_direction: Vec3,
    time: Real,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self::new_at_time(origin, direction, 0.0)
    }

    pub fn new_at_time(origin: Point3, direction: Vec3, time: Real) -> Self {
        let inv_direction = Vec3::new(
            1.0 / direction.x(),
            1.0 / direction.y(),
            1.0 / direction.z(),
        );
        Self {
            origin,
            direction,
            inv_direction,
            time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> Real {
        self.time
    }

    pub fn inv_direction(&self) -> Vec3 {
        self.inv_direction
    }

    pub fn at(&self, t: Real) -> Point3 {
        self.origin + t * self.direction
    }
}
