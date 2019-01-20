#![allow(dead_code)]

use std::f64;

use crate::geometry::{Vector3, Ray};

pub struct HitRecord {
    pub t: f64,
    pub point: Vector3,
    pub normal: Vector3,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitWorld<'a> {
    targets: Vec<Box<Hit + 'a>>,
}

impl<'a> HitWorld<'a> {
    pub fn new() -> HitWorld<'a> {
        HitWorld {targets: vec![]}
    }

    pub fn add<T: Hit + 'a>(&mut self, target: T) {
        self.targets.push(Box::new(target));
    }
}

impl<'a> Hit for HitWorld<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut h: Option<HitRecord> = None;

        for target in self.targets.iter() {
            if let Some(r) = target.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = r.t;
                h = Some(r);
            }
        }

        if hit_anything {
            h
        } else {
            None
        }
    }
}

pub fn ray_color(r: &Ray, target: &Hit) -> Vector3 {
    if let Some(h) = target.hit(r, 0.0, f64::MAX) {
        Vector3::new(h.normal.x + 1.0, h.normal.y + 1.0, h.normal.z + 1.0) * 0.5
    } else {
        let t = 0.5 * (r.direction.y + 1.0);
        Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
    }
}
