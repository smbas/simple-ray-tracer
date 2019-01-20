#![allow(dead_code)]

use crate::geometry::{Vector3, Ray};

pub fn ray_color(r: &Ray) -> Vector3 {
    let t = 0.5 * (r.direction.y + 1.0);
    Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
}
