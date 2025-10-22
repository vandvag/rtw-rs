use glam::DVec3;

use crate::{
    hittable::HitRecord,
    material::{Material, Scatter},
    ray::Ray,
    utils::vec::reflect,
};

pub struct Metal {
    pub albedo: DVec3,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(ray.direction, hr.normal);
        let scattered = Ray::new(hr.point, reflected);
        let attenuation = self.albedo;
        Some(Scatter {
            scattered,
            attenuation,
        })
    }
}

