use glam::DVec3;

use crate::{
    hittable::HitRecord,
    material::{Material, Scatter},
    ray::Ray,
    utils::vec::{reflect, refract},
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
        let cos_theta = f64::min(1.0, hr.normal.dot(-unit_direction));
        let sin_theta = (1.0 - cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract {
            reflect(unit_direction, hr.normal)
        } else {
            refract(unit_direction, hr.normal, ri)
        };

        let scattered = Ray::new(hr.point, direction);

        Some(Scatter {
            scattered,
            attenuation,
        })
    }
}
