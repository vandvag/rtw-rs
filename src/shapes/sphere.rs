use crate::ray::Ray;
use glam::DVec3;

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn hit(self, ray: &Ray) -> f64 {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(ray.direction);
        let b = -2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}
