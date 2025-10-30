use crate::{
    ray::Ray,
    utils::interval::{Interval, IntervalExtend, New},
    Result,
};
use glam::DVec3;
use rand::seq::IteratorRandom;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Clone, Copy)]
pub(crate) enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn random() -> Self {
        // TODO: Handle unwrap
        Axis::iter().choose(&mut rand::rng()).unwrap()
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: DVec3, b: DVec3) -> Result<Self> {
        let x = Interval::new(a.x, b.x)?;
        let y = Interval::new(a.y, b.y)?;
        let z = Interval::new(a.z, b.z)?;

        Ok(Self::new(x, y, z))
    }

    pub fn from_aabbs(box1: &Self, box2: &Self) -> Self {
        Self::new(
            Interval::from_ranges(&box1.x, &box2.x),
            Interval::from_ranges(&box1.y, &box2.y),
            Interval::from_ranges(&box1.z, &box2.z),
        )
    }

    pub fn interval(&self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x.clone(),
            Axis::Y => self.y.clone(),
            Axis::Z => self.z.clone(),
        }
    }

    pub fn hit(&self, ray: &Ray, interval: Interval) -> Option<Interval> {
        let mut t_start = interval.start;
        let mut t_end = interval.end;

        for axis in Axis::iter() {
            let axis_interval = self.interval(axis);
            let adinv = match axis {
                Axis::X => 1.0 / ray.direction.x,
                Axis::Y => 1.0 / ray.direction.y,
                Axis::Z => 1.0 / ray.direction.z,
            };
            let orig = match axis {
                Axis::X => ray.origin.x,
                Axis::Y => ray.origin.y,
                Axis::Z => ray.origin.z,
            };

            let t0 = (axis_interval.start - orig) * adinv;
            let t1 = (axis_interval.end - orig) * adinv;

            if t0 < t1 {
                if t0 > t_start {
                    t_start = t0;
                }
                if t1 < t_end {
                    t_end = t1;
                }
            } else {
                if t1 > t_start {
                    t_start = t1;
                }
                if t0 < t_end {
                    t_end = t0;
                }
            }

            if t_end <= t_start {
                return None;
            }
        }
        Some(t_start..t_end)
    }
}
