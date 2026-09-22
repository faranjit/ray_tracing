use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable, HittableList, Object},
    interval::Interval,
    ray::Ray,
};

#[derive(Clone)]
pub struct BVHNode {
    left: Box<Object>,
    right: Box<Object>,
    bbox: Aabb,
    axis: usize,
}

impl BVHNode {
    pub fn build(objects: &mut [Object]) -> Object {
        if objects.len() == 1 {
            return objects[0].clone();
        }

        let mut bbox = Aabb::empty();

        for o in objects.iter() {
            bbox = Aabb::from_boxes(&bbox, &o.bounding_box());
        }
        let axis = bbox.longest_axis();

        objects.sort_unstable_by(|a, b| {
            let a_min = a.bounding_box().axis_interval(axis).min();
            let b_min = b.bounding_box().axis_interval(axis).min();
            a_min
                .partial_cmp(&b_min)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mid = objects.len() / 2;
        let (l, r) = objects.split_at_mut(mid);

        Object::Bvh(BVHNode {
            left: Box::new(Self::build(l)),
            right: Box::new(Self::build(r)),
            bbox,
            axis,
        })
    }

    pub fn from_list(list: &mut HittableList) -> Object {
        Self::build(&mut list.objects)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        if !self.bbox.hit(r, &ray_t) {
            return None;
        }

        let (near, far) = if r.direction()[self.axis] >= 0.0 {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };

        let hit_near = near.hit(r, ray_t);
        let max = match &hit_near {
            None => ray_t.max(),
            Some(rec) => rec.t,
        };
        far.hit(r, Interval::new(ray_t.min(), max)).or(hit_near)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
