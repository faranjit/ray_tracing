use std::ops;

use crate::rtweekend::{Real, random_double, random_double_range};

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [Real; 3],
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    
    pub const fn new(e0: Real, e1: Real, e2: Real) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub const fn zero() -> Self {
        Self::ZERO
    }

    pub fn random() -> Self {
        Vec3 {
            e: [random_double(), random_double(), random_double()],
        }
    }

    pub fn random_range(min: Real, max: Real) -> Self {
        Vec3 {
            e: [
                random_double_range(min, max),
                random_double_range(min, max),
                random_double_range(min, max),
            ],
        }
    }

    pub fn x(&self) -> Real {
        self.e[0]
    }

    pub fn y(&self) -> Real {
        self.e[1]
    }

    pub fn z(&self) -> Real {
        self.e[2]
    }

    pub fn len_squared(&self) -> Real {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn len(&self) -> Real {
        self.len_squared().sqrt()
    }

    pub fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    #[inline]
    pub fn dot(&self, rhs: Vec3) -> Real {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    #[inline]
    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ],
        }
    }

    #[inline]
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    #[inline]
    pub fn refract(&self, n: Vec3, etai_over_etat: Real) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + (cos_theta * n));
        let r_out_parallel = -((1.0 - r_out_perp.len_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }
}

#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.len()
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.len_squared();
        if 1e-130 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

#[inline]
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double_range(-1.0, 1.0),
            random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<Real> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Real) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Mul<Vec3> for Real {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<Real> for Vec3 {
    fn mul_assign(&mut self, rhs: Real) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::Div<Real> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Real) -> Self::Output {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl ops::DivAssign<Real> for Vec3 {
    fn div_assign(&mut self, rhs: Real) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = Real;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}
