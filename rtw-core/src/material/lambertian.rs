use crate::{
    hittable::HitRecord,
    material::{Material, Scatter},
    ray::Ray,
    texture::{Texture, solid::SolidColor},
    utils::vec::random_unit_vector,
};
use glam::DVec3;
use std::sync::Arc;

pub struct Lambertian {
    pub texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(albedo: DVec3) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hr.normal + random_unit_vector();
        if scatter_direction.abs_diff_eq(DVec3::ZERO, 1e-8) {
            scatter_direction = hr.normal;
        }

        let attenuation = self.texture.value(hr.u, hr.v, hr.point);
        let scattered = Ray::with_time(hr.point, scatter_direction, ray.time);

        Some(Scatter {
            scattered,
            attenuation,
        })
    }
}
