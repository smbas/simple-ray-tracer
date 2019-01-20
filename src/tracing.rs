#![allow(dead_code)]

use crate::geometry::{Vector3, Ray};

pub fn ray_color(r: &Ray) -> Vector3 {
    if hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, &r) {
        Vector3::new(1.0, 0.0, 0.0)
    } else {
        let t = 0.5 * (r.direction.y + 1.0);
        Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
    }
}

fn hit_sphere(center: &Vector3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin  - *center;
    let a = Vector3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * Vector3::dot(&ray.direction, &oc);
    let c = Vector3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
