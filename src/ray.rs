use glam::DVec3;

use crate::hittable::sphere::Sphere;

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

    pub fn color(&self) -> DVec3 {
        let sph = Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5);
        let t = sph.hit(self);
        if t > 0.0 {
            let N = self.at(t) - DVec3::new(0.0, 0.0, -1.0);
            return 0.5 * DVec3::new(N.x + 1.0, N.y + 1.0, N.z + 1.0);
        }

        let unit_dir = self.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        DVec3::lerp(DVec3::ONE, DVec3::new(0.5, 0.7, 1.0), a)
    }
}
