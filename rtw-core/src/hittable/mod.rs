pub(crate) mod bvh_node;
pub(crate) mod list;
pub(crate) mod sphere;

use crate::{aabb::Aabb, material::Material, ray::Ray, utils::interval::Interval};
use glam::DVec3;
use std::sync::Arc;

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
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
            u: 0.0,
            v: 0.0,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> Aabb;
}
