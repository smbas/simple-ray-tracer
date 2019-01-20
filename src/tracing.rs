#![allow(dead_code)]

use crate::geometry::{Vector3, Ray};

pub fn ray_color(r: &Ray) -> Vector3 {
    if let Some(p) = hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, &r) {
        let n = p - Vector3::new(0.0, 0.0, -1.0);
        Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
    } else {
        let t = 0.5 * (r.direction.y + 1.0);
        Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
    }
}

fn hit_sphere(center: &Vector3, radius: f64, ray: &Ray) -> Option<Vector3> {
    let oc = ray.origin  - *center;
    let a = Vector3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * Vector3::dot(&ray.direction, &oc);
    let c = Vector3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        Some(ray.point_at(t))
    }
}
