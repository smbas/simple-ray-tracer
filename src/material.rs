use rand::prelude::*;

use crate::geometry::{Vector3, Ray};
use crate::tracing::{HitRecord};
use crate::utils;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vector3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Vector3, Ray)> {
        let tangent = hit.point + hit.normal + utils::random_in_unit_sphere();
        let scattered = Ray::new(hit.point, tangent - hit.point);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Vector3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vector3, Ray)> {
        let reflected = Vector3::reflect(&ray.direction, &hit.normal);
        let scattered = Ray::new(hit.point, reflected + utils::random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;

        if Vector3::dot(&scattered.direction, &hit.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric {refractive_index}
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vector3, Ray)> {
        let reflected = Vector3::reflect(&ray.direction, &hit.normal);
        let d = Vector3::dot(&ray.direction, &hit.normal);
        let ni_over_nt = if d > 0.0 { 
            self.refractive_index 
        } else { 
            1.0 / self.refractive_index 
        };
        let outward_normal = if d > 0.0 { 
            -hit.normal 
        } else { 
            hit.normal 
        };
        let cosine = if d > 0.0 { 
            self.refractive_index * Vector3::dot(&ray.direction, &hit.normal)
        } else {
            -Vector3::dot(&ray.direction, &hit.normal)
        };
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        if let Some(refracted) = Vector3::refract(&ray.direction, &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.refractive_index);
            if random::<f64>() < reflect_prob {
                return Some((attenuation, Ray::new(hit.point, reflected)));
            } else {
                return Some((attenuation, Ray::new(hit.point, refracted)));
            }
        } else {
            return Some((attenuation, Ray::new(hit.point, reflected)));
        }
    }
}

pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
