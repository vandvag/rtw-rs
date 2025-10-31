use glam::DVec3;

use crate::texture::{Texture, solid::SolidColor};
use std::sync::Arc;

pub struct Checker {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: scale.recip(),
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, color1: DVec3, color2: DVec3) -> Self {
        Self {
            inv_scale: scale.recip(),
            even: Arc::new(SolidColor::new(color1)),
            odd: Arc::new(SolidColor::new(color2)),
        }
    }
}
impl Texture for Checker {
    fn value(&self, u: f64, v: f64, point: glam::DVec3) -> glam::DVec3 {
        let x_int = (self.inv_scale * point.x).floor() as i32;
        let y_int = (self.inv_scale * point.y).floor() as i32;
        let z_int = (self.inv_scale * point.z).floor() as i32;

        if (x_int + y_int + z_int) % 2 == 0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
