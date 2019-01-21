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

