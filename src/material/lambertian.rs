use glam::DVec3;

use crate::{hittable::HitRecord, material::{Material, Scatter}, ray::Ray, utils::vec::random_unit_vector};

pub struct Lambertian {
    pub albedo: DVec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hr.normal + random_unit_vector();
        if scatter_direction.abs_diff_eq(DVec3::ZERO, 1e-8) {
            scatter_direction = hr.normal;
        }

        let attenuation = self.albedo;
        let scattered = Ray::new(hr.point, scatter_direction);

        Some(Scatter {
            scattered,
            attenuation
        })
    }
}
