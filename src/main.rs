mod geometry;
mod tracing;
mod material;
mod utils;

use rand::prelude::*;

use geometry::{Vector3, Sphere};
use tracing::{HitWorld, Camera};
use material::{Lambertian, Metal, Dielectric};

fn main() {
    let nx = 800;
    let ny = 400;
    let ns = 20;

    println!("P3\n{} {}\n255", nx, ny);

    let world = random_scene();
    let lookfrom = Vector3::new(8.0, 2.0, 3.0);
    let lookat = Vector3::new(4.0, 1.0, 2.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
        (nx as f64) / (ny as f64),
        0.1,
        (lookat - lookfrom).length(),
    );

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (x as f64 + random::<f64>()) / (nx as f64);
                let v = (y as f64 + random::<f64>()) / (ny as f64);
                let r = camera.get_ray(u, v);
                color += tracing::ray_color(&r, &world, 0.0);
            }

            color /= ns as f64;
            color = Vector3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());

            let ir = (255.99f64 * color.x) as i32;
            let ig = (255.99f64 * color.y) as i32;
            let ib = (255.99f64 * color.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn random_scene<'a>() -> HitWorld<'a> {
    let mut world = HitWorld::new();
    world.add(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(Vector3::new(0.5, 0.5, 0.5))));
    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let mat: f64 = rng.gen();
            let center = Vector3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat < 0.8 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(Vector3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>()
                        ))
                    ));
                } else if mat < 0.95 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Metal::new(Vector3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>()
                        )
                    ));
                } else {
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.add(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    world.add(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Vector3::new(0.4, 0.2, 0.1))));
    world.add(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)));

    world
}
