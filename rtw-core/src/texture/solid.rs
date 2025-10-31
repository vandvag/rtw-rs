use glam::DVec3;

use crate::texture::Texture;

pub struct SolidColor {
    albedo: DVec3,
}

impl SolidColor {
    pub fn new(albedo: DVec3) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: DVec3) -> DVec3 {
        self.albedo
    }
}
