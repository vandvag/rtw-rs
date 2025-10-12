use crate::TOL;
use core::f64;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn len(&self) -> f64 {
        self.len2().sqrt()
    }

    pub fn len2(&self) -> f64 {
        self.dot(self)
    }

    pub fn unit(&self) -> Self {
        let l = self.len();
        if approx_eq(l, 0.0, TOL) {
            return Self::zero();
        }

        Self {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x, TOL)
            && approx_eq(self.y, other.y, TOL)
            && approx_eq(self.z, self.z, TOL)
    }
}

pub struct Color(pub Vec3);

impl Color {
    pub fn r(&self) -> f64 {
        self.0.x
    }

    pub fn g(&self) -> f64 {
        self.0.y
    }

    pub fn b(&self) -> f64 {
        self.0.z
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = (255.999 * self.r()) as u8;
        let g = (255.999 * self.g()) as u8;
        let b = (255.999 * self.b()) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(32.0, v1.dot(&v2));
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(Vec3::new(-3.0, 6.0, 3.0), v1.cross(&v2))
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.len(), 5.0);
    }

    #[test]
    fn test_unit() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let unit = v.unit();
        let expected = Vec3::new(0.6, 0.8, 0.0);

        assert_eq!(unit, expected);
        assert!(approx_eq(unit.len(), 1.0, TOL));
    }

    #[test]
    fn test_neg() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(Vec3::new(-1.0, -2.0, -3.0), -v1);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let sum = v1 + v2;

        assert_eq!(sum, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_add_f64() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 5.0;

        let sum = v1 + scalar;

        assert_eq!(sum, Vec3::new(6.0, 7.0, 8.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let res = v2 - v1;

        assert_eq!(res, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_sub_f64() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 3.0;

        let res = v1 - scalar;

        assert_eq!(res, Vec3::new(-2.0, -1.0, 0.0));
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 5.0;

        let sum = v1 * scalar;

        assert_eq!(sum, Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(5.0, 10.0, 15.0);
        let scalar = 5.0;

        let sum = v1 / scalar;

        assert_eq!(sum, Vec3::new(1.0, 2.0, 3.0));
    }
}
