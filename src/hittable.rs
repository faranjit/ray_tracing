use crate::{
    aabb::AABB,
    bvh_node::BVHNode,
    interval::Interval,
    material::Material,
    ray::Ray,
    rtweekend::PI,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Material,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
        mat: Material,
    ) -> Self {
        let normal = if front_face { normal } else { -normal };

        Self {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            mat,
        }
    }
}

#[derive(Clone)]
pub enum Object {
    Sphere(Sphere),
    Bvh(BVHNode),
}

impl Object {
    pub fn sphere(center: Point3, radius: f64, mat: Material) -> Self {
        Object::Sphere(Sphere::new(center, radius, mat))
    }

    pub fn moving_sphere(center1: Point3, center2: Point3, radius: f64, mat: Material) -> Self {
        Object::Sphere(Sphere::moving_sphere(center1, center2, radius, mat))
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}

impl Hittable for Object {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Object::Sphere(s) => s.hit(r, ray_t),
            Object::Bvh(b) => b.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> AABB {
        match self {
            Object::Sphere(s) => s.bbox,
            Object::Bvh(b) => b.bbox(),
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Material,
    bbox: AABB,
}

impl Sphere {
    // Stationary sphere
    pub fn new(center: Point3, radius: f64, mat: Material) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AABB::from_points(center - rvec, center + rvec);

        Self {
            center: Ray::new(center, Vec3::new(0.0, 0.0, 0.0)),
            radius,
            mat,
            bbox,
        }
    }

    // Moving sphere
    pub fn moving_sphere(center1: Point3, center2: Point3, radius: f64, mat: Material) -> Self {
        let center = Ray::new(center1, center2 - center1);

        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = AABB::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        let bbox = AABB::from_boxes(&box1, &box2);

        Self {
            center: Ray::new(center1, center2 - center1),
            radius,
            mat,
            bbox,
        }
    }

    // A function to help find the center of the sphere at a given time
    pub fn center_at(&self, time: f64) -> Point3 {
        self.center.at(time)
    }
}

pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;
    (phi / (2.0 * PI), theta / PI)
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center_at(r.time());
        let oc = current_center - r.origin();
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
        let outward_normal = (p - current_center) / self.radius;
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let (u, v) = get_sphere_uv(outward_normal);
        let hit_record = HitRecord::new(p, outward_normal, t, u, v, front_face, self.mat.clone());

        Some(hit_record)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct HittableList {
    pub objects: Vec<Object>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::empty(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.bbox = AABB::from_boxes(&self.bbox, &object.bounding_box());
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

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
