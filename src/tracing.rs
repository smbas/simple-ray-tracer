#![allow(dead_code)]

use std::f64;

use crate::geometry::{Vector3, Ray};
use crate::material::{Material};

pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vector3,
    pub normal: Vector3,
    pub material: &'a Material
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

pub fn ray_color(r: &Ray, target: &Hit, depth: f64) -> Vector3 {
    if let Some(h) = target.hit(r, 0.001, f64::MAX) {
        if depth < 50.0 {
            if let Some((attenuation, scattered)) = h.material.scatter(&r, &h) {
                return attenuation * ray_color(&scattered, target, depth + 1.0)
            }
        }
        return Vector3::new(0.0, 0.0, 0.0)
    } else {
        let t = 0.5 * (r.direction.y + 1.0);
        Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
    }
}

#[derive(Debug)]
pub struct Camera {
    pub origin: Vector3,
    pub lower_left_corner: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
}

impl Camera {
    pub fn new(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalized();
        let u = Vector3::cross(&vup, &w).normalized();
        let v = Vector3::cross(&w, &u);

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - u * half_width - v * half_height - w,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}
