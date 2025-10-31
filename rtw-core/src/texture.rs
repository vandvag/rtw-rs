pub mod checker;
pub mod solid;

use glam::DVec3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: DVec3) -> DVec3;
}
