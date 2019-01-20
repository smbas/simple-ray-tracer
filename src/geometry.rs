#![allow(dead_code)]

use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {x, y, z}
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        *self = *self / self.length();
    }

    pub fn normalized(&self) -> Vector3 {
        let mut n = *self;
        n.normalize();
        n
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, f: f64) -> Vector3 {
        Vector3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, f: f64) {
        *self = Vector3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        };
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, f: f64) -> Vector3 {
        Vector3 {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, f: f64) {
        *self = Vector3 {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_ops() {
        assert_eq!(Vector3::new(0.0, 1.0, 2.0) + Vector3::new(-2.0, 3.0, 4.0), Vector3::new(-2.0, 4.0, 6.0));
        assert_eq!(Vector3::new(0.0, 1.0, 2.0) - Vector3::new(-2.0, 3.0, 4.0), Vector3::new(2.0, -2.0, -2.0));
        assert_eq!(Vector3::new(2.0, 3.0, 4.0) / 2.0, Vector3::new(1.0, 1.5, 2.0));
        assert_eq!(Vector3::new(2.0, 3.0, 4.0) * 2.0, Vector3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_vector3_fns() {
        assert_eq!(Vector3::new(1.0, 2.0, 3.0).squared_length(), 14.0);
        assert_eq!(Vector3::new(3.0, 4.0, 5.0).length(), 50.0f64.sqrt());
        assert_eq!(Vector3::new(1.0, 2.0, 3.0).normalized().length(), 1.0);
    }
}
