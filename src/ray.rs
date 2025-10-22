use std::f64;

use crate::hittable::Hittable;
use glam::DVec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        t * self.direction + self.origin
    }

    pub fn color<T>(&self, world: &T, depth: u32) -> DVec3
    where
        T: Hittable,
    {
        if depth == 0 {
            return DVec3::ZERO;
        }

        if let Some(hr) = world.hit(self, 0.001..f64::INFINITY) {
            let direction = hr.normal + crate::random_unit_vector();
            return 0.1 * Ray::color(&Ray::new(hr.point, direction), world, depth - 1);
        }

        let unit_dir = self.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        DVec3::lerp(DVec3::ONE, DVec3::new(0.5, 0.7, 1.0), a)
    }
}
