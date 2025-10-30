use crate::{
    aabb::{Aabb, Axis},
    hittable::{list::HittableList, HitRecord, Hittable},
    ray::Ray,
    utils::interval::Interval,
};
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl From<HittableList> for BvhNode {
    fn from(value: HittableList) -> Self {
        let mut list = value;
        let size = list.size();
        BvhNode::from_hittables(list.objects_mut(), 0, size)
    }
}

impl BvhNode {
    fn from_hittables(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let axis = Axis::random();

        let object_span = end - start;

        match object_span {
            1 => {
                let left = objects[start].clone();
                let right = objects[start].clone();
                let bbox = Aabb::from_aabbs(&left.bounding_box(), &right.bounding_box());
                Self { left, right, bbox }
            }
            2 => {
                let left = objects[start].clone();
                let right = objects[start + 1].clone();
                let bbox = Aabb::from_aabbs(&left.bounding_box(), &right.bounding_box());
                Self { left, right, bbox }
            }
            _ => {
                objects[start..end].sort_by(|a, b| {
                    let a_axis_range = a.bounding_box().interval(axis);
                    let b_axis_range = b.bounding_box().interval(axis);
                    a_axis_range.start.total_cmp(&b_axis_range.start)
                });
                let mid = start + object_span / 2;
                // TODO: avoid this clone
                let left = Arc::new(BvhNode::from_hittables(objects, start, mid));
                let right = Arc::new(BvhNode::from_hittables(objects, mid, end));
                let bbox = Aabb::from_aabbs(&left.bounding_box(), &right.bounding_box());
                Self { left, right, bbox }
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let _int = self.bbox.hit(ray, interval.clone())?;

        let hr_left = self.left.hit(ray, interval.clone());
        let right_interval = if let Some(ll) = &hr_left {
            interval.start..ll.t
        } else {
            interval
        };

        let hr_right = self.right.hit(ray, right_interval);

        match (hr_left, hr_right) {
            (Some(l), Some(r)) => {
                if r.t < l.t {
                    Some(r)
                } else {
                    Some(l)
                }
            }
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
