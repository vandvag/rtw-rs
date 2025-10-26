pub mod dielectric;
pub mod lambertian;
pub mod metal;

use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: DVec3,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter>;
}
