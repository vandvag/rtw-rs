use glam::DVec3;

use crate::{
    hittable::HitRecord,
    material::{Material, Scatter},
    ray::Ray,
    utils::vec::{random_unit_vector, reflect},
};

// TODO: fuzz needs to be in [0, 1]
pub struct Metal {
    pub albedo: DVec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        let reflected =
            reflect(ray.direction, hr.normal).normalize() + self.fuzz * random_unit_vector();
        let scattered = Ray::new(hr.point, reflected);
        let attenuation = self.albedo;
        if scattered.direction.dot(hr.normal) > 0.0 {
            Some(Scatter {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}

