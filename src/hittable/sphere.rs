use std::ops::{Range};
use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use glam::DVec3;

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        assert!(radius >= 0.0, "Sphere radius must be non-negative");
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discr_sqrt = discriminant.sqrt();
        // Find the nearest root
        let mut root = (h - discr_sqrt) / a;
        if !interval.contains(&root) {
            root = (h + discr_sqrt) / a;
            if !interval.contains(&root) {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord::init(p, normal, root, ray, self.mat.clone()))
    }
}
