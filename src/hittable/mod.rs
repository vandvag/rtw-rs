pub mod sphere;

use glam::DVec3;
use std::{ops::Range, sync::Arc};

use crate::{material::Material, ray::Ray};

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn init(
        point: DVec3,
        outward_normal: DVec3,
        t: f64,
        ray: &Ray,
        mat: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            point,
            normal,
            t,
            front_face,
            material: mat,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord>;
}

impl<T> Hittable for Vec<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let (_closest_so_far, hr) = self.iter().fold((interval.end, None), |acc, object| {
            if let Some(tmp) = object.hit(ray, interval.start..acc.0) {
                (tmp.t, Some(tmp))
            } else {
                acc
            }
        });

        hr
    }
}
