pub mod lambertian;
pub mod metal;
pub mod dielectric;

use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: DVec3
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter>;
}
