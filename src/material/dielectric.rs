use glam::DVec3;

use crate::{
    hittable::HitRecord,
    material::{Material, Scatter},
    ray::Ray,
    utils::vec::refract,
};

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        let attenuation = DVec3::ONE;
        let ri = if hr.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.normalize();
        let refracted = refract(unit_direction, hr.normal, ri);

        let scattered = Ray::new(hr.point, refracted);

        Some(Scatter {
            scattered,
            attenuation,
        })
    }
}
