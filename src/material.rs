use crate::{
    color::{BLACK, Color},
    hittable::HitRecord,
    ray::Ray,
    rtweekend::{Real, random_double},
    texture::{SolidColor, Texture},
    vec3::{Point3, random_unit_vector, unit_vector},
};

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    Diffuse(DiffuseLight),
    Isotropic(Isotropic),
}

impl Material {
    pub fn needs_uv(&self) -> bool {
        match self {
            Material::Lambertian(l) => l.tex.needs_uv(),
            Material::Diffuse(d) => d.tex.needs_uv(),
            Material::Isotropic(i) => i.tex.needs_uv(),
            Material::Metal(_) | Material::Dielectric(_) => false,
        }
    }
    
    pub fn lambertian(albedo: Color) -> Self {
        Material::Lambertian(Lambertian::from_color(albedo))
    }

    pub fn lambertian_texture(tex: Texture) -> Self {
        Material::Lambertian(Lambertian::new(tex))
    }

    pub fn metal(albedo: Color, fuzz: Real) -> Self {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    pub fn dielectric(refraction_index: Real) -> Self {
        Material::Dielectric(Dielectric::new(refraction_index))
    }

    pub fn diffuse_light(color: Color) -> Self {
        Material::Diffuse(DiffuseLight::new(Texture::solid_color(color)))
    }

    pub fn isotropic(color: Color) -> Self {
        Material::Isotropic(Isotropic::new(color))
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dielectric(d) => d.scatter(r_in, rec),
            Material::Diffuse(d) => d.scatter(r_in, rec),
            Material::Isotropic(i) => i.scatter(r_in, rec),
        }
    }

    pub fn emitted(&self, u: Real, v: Real, p: Point3) -> Color {
        match self {
            Material::Diffuse(d) => d.emitted(u, v, p),
            _ => BLACK,
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
    fuzz: Real,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Real) -> Self {
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
    refraction_index: Real,
    attenuation: Color,
}

impl Dielectric {
    const ATTENUATION: Color = Color::new(1.0, 1.0, 1.0);

    pub fn new(refraction_index: Real) -> Self {
        Self {
            refraction_index,
            attenuation: Self::ATTENUATION,
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
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };

        Some((
            self.attenuation,
            Ray::new_at_time(rec.p, direction, r_in.time()),
        ))
    }

    fn reflectance(cosine: Real, refraction_index: Real) -> Real {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        let x = 1.0 - cosine;
        let x2 = x * x;
        r0 + (1.0 - r0) * x2 * x2 * x
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Box<Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Texture) -> Self {
        Self { tex: Box::new(tex) }
    }

    pub fn emitted(&self, u: Real, v: Real, p: Point3) -> Color {
        self.tex.value(u, v, p)
    }

    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

#[derive(Clone)]
pub struct Isotropic {
    tex: Box<Texture>,
}

impl Isotropic {
    pub fn new(color: Color) -> Self {
        Self {
            tex: Box::new(Texture::solid_color(color)),
        }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new_at_time(rec.p, random_unit_vector(), r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}
