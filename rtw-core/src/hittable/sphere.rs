use std::ops::Range;
use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use glam::DVec3;

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
    pub new_center: Option<DVec3>,
}

impl Sphere {
    pub fn stationary(center: DVec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        assert!(radius >= 0.0, "Sphere radius must be non-negative");
        Self {
            center,
            radius,
            mat,
            new_center: None,
        }
    }

    pub fn moving(center: DVec3, new_center: DVec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
            new_center: Some(new_center),
        }
    }

    fn current_center(&self, ray: &Ray) -> DVec3 {
        if let Some(center2) = self.new_center {
            if let Some(current_time) = ray.time {
                self.center + current_time * center2
            } else { 
                self.center
            }
        } else {
            self.center
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let current_center = self.current_center(ray);
        let oc = current_center - ray.origin;
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
        let normal = (p - current_center) / self.radius;
        Some(HitRecord::init(p, normal, root, ray, self.mat.clone()))
    }
}
