use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray, utils::interval::Interval,
};
use std::sync::Arc;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.bbox = Aabb::from_aabbs(&self.bbox, &obj.bounding_box());
        self.objects.push(obj);
    }

    pub fn objects_mut(&mut self) -> &mut Vec<Arc<dyn Hittable>> {
        &mut self.objects
    }

    pub fn size(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let (_closest_so_far, hr) = self.objects.iter().fold((interval.end, None), |acc, obj| {
            if let Some(tmp) = obj.hit(ray, interval.start..acc.0) {
                (tmp.t, Some(tmp))
            } else {
                acc
            }
        });

        hr
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
