use std::ops;

use crate::rtweekend::{random_double, random_double_range};

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub const SPHERE_CENTER: Vec3 = Vec3::new(0.0, 0.0, -1.0);

    pub const fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn random() -> Self {
        Vec3 {
            e: [random_double(), random_double(), random_double()],
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3 {
            e: [
                random_double_range(min, max),
                random_double_range(min, max),
                random_double_range(min, max),
            ],
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn len_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    #[inline(always)]
    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    #[inline(always)]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ],
        }
    }

    #[inline]
    pub fn random_on_hemisphere(&self) -> Vec3 {
        let on_unit_sphere = random_unit_vector();
        if on_unit_sphere.dot(*self) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[inline(always)]
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }

    #[inline(always)]
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    #[inline(always)]
    pub fn refract(&self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -(self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + (cos_theta * n));
        let r_out_parallel = -((1.0 - r_out_perp.len_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }
}

#[inline(always)]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.len()
}

#[inline(always)]
pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.len_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

#[inline(always)]
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

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec3> for u64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self as f64
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl ops::Div<u64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: u64) -> Self::Output {
        let divider = rhs as f64;
        Vec3 {
            e: [
                self.e[0] / divider,
                self.e[1] / divider,
                self.e[2] / divider,
            ],
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
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
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}
