use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableList, Object},
    interval::Interval,
    ray::Ray,
    rtweekend::random_double,
};

#[derive(Clone)]
pub struct BVHNode {
    left: Box<Object>,
    right: Box<Object>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut [Object], start: usize, end: usize) -> Self {
        let mut bbox = AABB::empty();
        for object_index in start..end {
            bbox = AABB::from_boxes(&bbox, &objects[object_index].bounding_box())
        }

        let axis = bbox.longest_axis();

        let object_span = end - start;

        let left: Box<Object>;
        let right: Box<Object>;

        if object_span == 1 {
            left = Box::new(objects[start].clone());
            right = Box::new(objects[start].clone());
        } else if object_span == 2 {
            left = Box::new(objects[start].clone());
            right = Box::new(objects[start + 1].clone());
        } else {
            objects[start..end].sort_unstable_by(|a, b| {
                let a_min = a.bounding_box().axis_interval(axis).min();
                let b_min = b.bounding_box().axis_interval(axis).min();
                a_min
                    .partial_cmp(&b_min)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let mid = start + object_span / 2;
            left = Box::new(Object::Bvh(BVHNode::new(objects, start, mid)));
            right = Box::new(Object::Bvh(BVHNode::new(objects, mid, end)));
        }

        Self { left, right, bbox }
    }

    pub fn from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }

    pub fn bbox(&self) -> AABB {
        self.bbox
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, &ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);
        let max = match &hit_left {
            None => ray_t.max(),
            Some(rec) => rec.t,
        };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min(), max));
        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub fn box_compare(a: &Object, b: &Object, axis_index: usize) -> bool {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);
    a_axis_interval.min() < b_axis_interval.min()
}

pub fn box_x_compare(a: &Object, b: &Object) -> bool {
    box_compare(a, b, 0)
}

pub fn box_y_compare(a: &Object, b: &Object) -> bool {
    box_compare(a, b, 1)
}

pub fn box_z_compare(a: &Object, b: &Object) -> bool {
    box_compare(a, b, 2)
}
