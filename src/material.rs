use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    rtweekend::random_double,
    texture::{SolidColor, Texture},
    vec3::{random_unit_vector, unit_vector},
};

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn lambertian(albedo: Color) -> Self {
        Material::Lambertian(Lambertian::from_color(albedo))
    }

    pub fn lambertian_texture(tex: Texture) -> Self {
        Material::Lambertian(Lambertian::new(tex))
    }

    pub fn metal(albedo: Color, fuzz: f64) -> Self {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    pub fn dielectric(refraction_index: f64) -> Self {
        Material::Dielectric(Dielectric::new(refraction_index))
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dielectric(d) => d.scatter(r_in, rec),
        }
    }
}

#[derive(Clone)]
pub struct Lambertian {
    tex: Box<Texture>,
}

impl Lambertian {
    pub fn new(tex: Texture) -> Self {
        Self { tex: Box::new(tex) }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Box::new(Texture::SolidColor(SolidColor::new(albedo))),
        }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new_at_time(rec.p, scatter_direction, r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}

#[derive(Clone, Copy)]
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

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected =
            unit_vector(r_in.direction().reflect(rec.normal)) + (self.fuzz * random_unit_vector());
        let scattered = Ray::new_at_time(rec.p, reflected, r_in.time());
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
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

        Some((
            self.attenuation,
            Ray::new_at_time(rec.p, direction, r_in.time()),
        ))
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        (r0 * r0) + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
