use crate::{
    aabb::Aabb,
    bvh_node::BVHNode,
    color::Color,
    interval::{Interval, UNIVERSE},
    material::Material,
    ray::Ray,
    rtweekend::{INFINITY, PI, Real, degrees_to_radians, random_double},
    vec3::{Point3, Vec3, unit_vector},
};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Real,
    pub u: Real,
    pub v: Real,
    pub front_face: bool,
    pub mat: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        t: Real,
        u: Real,
        v: Real,
        r: &Ray,
        mat: &'a Material,
    ) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
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

    pub fn transformed(self, p: Point3, normal: Vec3) -> Self {
        Self { p, normal, ..self }
    }
}

#[derive(Clone)]
pub enum Object {
    Sphere(Sphere),
    Bvh(BVHNode),
    Quad(Quad),
    List(HittableList),
    Translate(Box<Translate<Object>>),
    RotateY(Box<RotateY<Object>>),
    Medium(Box<ConstantMedium<Object>>),
}

impl Object {
    pub fn sphere(center: Point3, radius: Real, mat: Material) -> Self {
        Object::Sphere(Sphere::new(center, radius, mat))
    }

    pub fn moving_sphere(center1: Point3, center2: Point3, radius: Real, mat: Material) -> Self {
        Object::Sphere(Sphere::moving_sphere(center1, center2, radius, mat))
    }

    pub fn quad(q: Point3, u: Vec3, v: Vec3, mat: Material) -> Self {
        Object::Quad(Quad::new(q, u, v, mat))
    }

    pub fn list(list: HittableList) -> Self {
        Object::List(list)
    }

    pub fn translate(object: Object, offset: Vec3) -> Self {
        Object::Translate(Box::new(Translate::new(object, offset)))
    }

    pub fn rotate_y(object: Object, angle: Real) -> Self {
        Object::RotateY(Box::new(RotateY::new(object, angle)))
    }

    pub fn medium_color(boundary: Object, density: Real, color: Color) -> Self {
        Object::Medium(Box::new(ConstantMedium::from_color(
            boundary, density, color,
        )))
    }

    pub fn box_3d(a: Point3, b: Point3, mat: Material) -> Self {
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        let mut sides = HittableList::new();
        let (x0, y0, z0) = (min.x(), min.y(), min.z());
        let (x1, y1, z1) = (max.x(), max.y(), max.z());

        sides.add(Object::quad(Point3::new(x0, y0, z1), dx, dy, mat.clone())); // front
        sides.add(Object::quad(Point3::new(x1, y0, z1), -dz, dy, mat.clone())); // right
        sides.add(Object::quad(Point3::new(x1, y0, z0), -dx, dy, mat.clone())); // back
        sides.add(Object::quad(Point3::new(x0, y0, z0), dz, dy, mat.clone())); // left
        sides.add(Object::quad(Point3::new(x0, y1, z1), dx, -dz, mat.clone())); // top
        sides.add(Object::quad(Point3::new(x0, y0, z0), dx, dz, mat)); // bottom

        Object::list(sides)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>>;

    fn bounding_box(&self) -> Aabb;
}

impl Hittable for Object {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        match self {
            Object::Sphere(s) => s.hit(r, ray_t),
            Object::Bvh(b) => b.hit(r, ray_t),
            Object::Quad(q) => q.hit(r, ray_t),
            Object::List(l) => l.hit(r, ray_t),
            Object::Translate(t) => t.hit(r, ray_t),
            Object::RotateY(ro) => ro.hit(r, ray_t),
            Object::Medium(m) => m.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> Aabb {
        match self {
            Object::Sphere(s) => s.bounding_box(),
            Object::Bvh(b) => b.bounding_box(),
            Object::Quad(q) => q.bounding_box(),
            Object::List(l) => l.bounding_box(),
            Object::Translate(t) => t.bounding_box(),
            Object::RotateY(ro) => ro.bounding_box(),
            Object::Medium(m) => m.bounding_box(),
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    center_vec: Vec3,
    radius: Real,
    mat: Material,
    bbox: Aabb,
}

impl Sphere {
    // Stationary sphere
    pub fn new(center: Point3, radius: Real, mat: Material) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::from_points(center - rvec, center + rvec);

        Self {
            center,
            center_vec: Vec3::zero(),
            radius,
            mat,
            bbox,
        }
    }

    // Moving sphere
    pub fn moving_sphere(center1: Point3, center2: Point3, radius: Real, mat: Material) -> Self {
        let center_vec = center2 - center1;

        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(center1 - rvec, center1 + rvec);
        let box2 = Aabb::from_points(center2 - rvec, center2 + rvec);
        let bbox = Aabb::from_boxes(&box1, &box2);

        Self {
            center: center1,
            center_vec,
            radius,
            mat,
            bbox,
        }
    }

    // A function to help find the center of the sphere at a given time
    pub fn center_at(&self, time: Real) -> Point3 {
        self.center + time * self.center_vec
    }
}

pub fn get_sphere_uv(p: Point3) -> (Real, Real) {
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
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
        let (u, v) = if self.mat.needs_uv() {
            get_sphere_uv(outward_normal)
        } else {
            (0.0, 0.0)
        };
        let hit_record = HitRecord::new(p, outward_normal, t, u, v, r, &self.mat);

        Some(hit_record)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Material,
    bbox: Aabb,
    normal: Vec3,
    d: Real,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Material) -> Self {
        let n = u.cross(v);
        let normal = unit_vector(n);
        let d = normal.dot(q);
        let w = n / n.len_squared();

        let bbox_diagonal1 = Aabb::from_points(q, q + u + v);
        let bbox_diagonal2 = Aabb::from_points(q + u, q + v);
        let bbox = Aabb::from_boxes(&bbox_diagonal1, &bbox_diagonal2);

        Self {
            q,
            u,
            v,
            mat,
            bbox,
            normal,
            d,
            w,
        }
    }

    fn is_interior(a: Real, b: Real) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        unit_interval.contains(a) && unit_interval.contains(b)
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let denom = self.normal.dot(r.direction());

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !Self::is_interior(alpha, beta) {
            return None;
        }

        let rec = HitRecord::new(intersection, self.normal, t, alpha, beta, r, &self.mat);

        Some(rec)
    }
}

#[derive(Clone)]
pub struct Translate<T: Hittable> {
    object: T,
    offset: Vec3,
    bbox: Aabb,
}

impl<T: Hittable> Translate<T> {
    pub fn new(object: T, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let offset_r = Ray::new_at_time(r.origin() - self.offset, r.direction(), r.time());

        self.object.hit(&offset_r, ray_t).map(|rec| {
            let p = rec.p + self.offset;
            let n = rec.normal;
            rec.transformed(p, n)
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

#[derive(Clone)]
pub struct RotateY<T: Hittable> {
    object: T,
    sin_theta: Real,
    cos_theta: Real,
    bbox: Aabb,
}

impl<T: Hittable> RotateY<T> {
    pub fn new(object: T, angle: Real) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as Real) * bbox.x.max() + ((1 - i) as Real) * bbox.x.min();
                    let y = (j as Real) * bbox.y.max() + ((1 - j) as Real) * bbox.y.min();
                    let z = (k as Real) * bbox.z.max() + ((1 - k) as Real) * bbox.z.min();

                    let new_x = (cos_theta * x) + (sin_theta * z);
                    let new_z = (-sin_theta) * x + (cos_theta * z);

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: Aabb::from_points(min, max),
        }
    }
}

impl<T: Hittable> Hittable for RotateY<T> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        // Transform the ray from world space to object space.
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new_at_time(origin, direction, r.time());

        // Determine whether an intersection exists in object space (and if so, where).
        self.object.hit(&rotated_r, ray_t).map(|rec| {
            let p = Point3::new(
                self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z(),
                rec.p.y(),
                -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z(),
            );
            let normal = Vec3::new(
                self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
                rec.normal.y(),
                -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z(),
            );
            rec.transformed(p, normal)
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

#[derive(Clone)]
pub struct ConstantMedium<T: Hittable> {
    boundary: T,
    neg_inv_density: Real,
    phase_function: Material,
}

impl<T: Hittable> ConstantMedium<T> {
    pub fn from_color(boundary: T, density: Real, color: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Material::isotropic(color),
        }
    }
}

impl<T: Hittable> Hittable for ConstantMedium<T> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let rec1 = self.boundary.hit(r, UNIVERSE)?;
        let rec2 = self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, INFINITY))?;

        let t_start = rec1.t.max(ray_t.min()).max(0.0);
        let t_end = rec2.t.min(ray_t.max());

        if t_start >= t_end {
            return None;
        }

        let ray_length = r.direction().len();
        let distance_inside_boundary = (t_end - t_start) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t_start + hit_distance / ray_length;

        Some(HitRecord {
            p: r.at(t),
            normal: Vec3::new(1.0, 0.0, 0.0), // arbitrary
            t,
            u: 0.0,
            v: 0.0,
            front_face: true, // also arbitrary
            mat: &self.phase_function,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Object>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::empty(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.bbox = Aabb::from_boxes(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut hit_anything: Option<HitRecord<'_>> = None;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min(), closest_so_far)) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
