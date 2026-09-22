use crate::{
    rtweekend::{Real, random_int_range}, vec3::{Point3, Vec3, unit_vector},
};

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let rand_vec = (0..POINT_COUNT)
            .map(|_| unit_vector(Vec3::random_range(-1.0, 1.0)))
            .collect::<Vec<Vec3>>()
            .try_into()
            .unwrap();
        Self {
            rand_vec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Point3) -> Real {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: i32) -> Real {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut perm = [0usize; POINT_COUNT];
        for (i, slot) in perm.iter_mut().enumerate() {
            *slot = i;
        }
        Self::permute(&mut perm);
        perm
    }
    
    fn permute(p: &mut [usize]) {
        for i in (1..p.len()).rev() {
            let target = random_int_range(0, i);
            p.swap(i, target);
        }
    }

    #[inline]
    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: Real, v: Real, w: Real) -> Real {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as Real, v - j as Real, w - k as Real);
                    accum += ((i as Real) * uu + (1 - i) as Real * (1.0 - uu))
                        * ((j as Real) * vv + (1 - j) as Real * (1.0 - vv))
                        * ((k as Real) * ww + (1 - k) as Real * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }
}
