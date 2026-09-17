use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    rtweekend::random_double,
    vec3::{random_unit_vector, unit_vector},
};

pub trait Material {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected =
            unit_vector(r_in.direction().reflect(rec.normal)) + (self.fuzz * random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
    attenuation: Color,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index,
            attenuation: Color::new(1.0, 1.0, 1.0),
        }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        (r0 * r0) + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > random_double() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };

        Some((self.attenuation, Ray::new(rec.p, direction)))
    }
}
