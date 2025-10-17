use glam::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn color(&self) -> DVec3 {
        let unit_dir = self.direction.normalize();
        let a = 0.5 * unit_dir.y + 1.0;
        DVec3::lerp(DVec3::ONE, DVec3::new(0.5, 0.7, 1.0), a)
    }
}
