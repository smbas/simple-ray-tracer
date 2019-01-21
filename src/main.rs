mod geometry;
mod tracing;

use rand::prelude::*;

use geometry::{Vector3, Sphere};
use tracing::{HitWorld, Camera};

fn main() {
    let nx = 800;
    let ny = 400;
    let ns = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let mut world = HitWorld::new();
    world.add(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new();

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (x as f64 + random::<f64>()) / (nx as f64);
                let v = (y as f64 + random::<f64>()) / (ny as f64);
                let r = camera.get_ray(u, v);
                color += tracing::ray_color(&r, &world);
            }

            color /= ns as f64;
            let ir = (255.99f64 * color.x) as i32;
            let ig = (255.99f64 * color.y) as i32;
            let ib = (255.99f64 * color.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
