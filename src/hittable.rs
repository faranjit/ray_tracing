use std::sync::Arc;

use crate::{
    interval::Interval, material::Material, ray::Ray, vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Material,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool, mat: Material) -> Self {
        let normal = if front_face { normal } else { -normal };

        Self {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }
}

pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn sphere(center: Point3, radius: f64, mat: Material) -> Arc<Self> {
        Arc::new(Object::Sphere(Sphere::new(center, radius, mat)))
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl Hittable for Object {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Object::Sphere(s) => s.hit(r, ray_t),
        }
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Material) -> Self {
        Self { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().len_squared();
        let h = r.direction().dot(oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let hit_record = HitRecord::new(p, outward_normal, t, front_face, self.mat.clone());

        return Some(hit_record);
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<Object>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<Object>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min(), closest_so_far)) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }
}
