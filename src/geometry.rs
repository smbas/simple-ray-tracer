#![allow(dead_code)]

use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use crate::tracing::{Hit, HitRecord};
use crate::material::{Material};

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

    pub fn dot(a: &Vector3, b: &Vector3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
        *v - *n * Vector3::dot(&v, &n) * 2.0
    }

    pub fn refract(v: &Vector3, n: &Vector3, ni_over_nt: f64) -> Option<Vector3> {
        let uv = v.normalized();
        let dt = Vector3::dot(&uv, &n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

        if discriminant > 0.0 {
            Some((uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt())
        } else {
            None
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {origin, direction: direction.normalized()}
    }

    pub fn point_at(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}

pub struct Sphere<'a> {
    pub center: Vector3,
    pub radius: f64,
    pub material: Box<Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new<T: 'a + Material>(center: Vector3, radius: f64, material: T) -> Sphere<'a> {
        Sphere {center, radius, material: Box::new(material)}
    }
}

impl<'a> Hit for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin  - self.center;
        let a = Vector3::dot(&ray.direction, &ray.direction);
        let b = Vector3::dot(&ray.direction, &oc);
        let c = Vector3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;
            if t <= t_min || t >= t_max {
                t = (-b + discriminant.sqrt()) / a;
            }
            if t > t_min && t < t_max {
                let p = ray.point_at(t);
                return Some(HitRecord {
                    t,
                    point: p,
                    normal: (p - self.center) / self.radius,
                    material: &*self.material
                });
            }
        }

        None
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
