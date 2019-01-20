mod geometry;
mod tracing;

use geometry::{Vector3, Ray, Sphere};
use tracing::{HitWorld};

fn main() {
    let nx = 800;
    let ny = 400;

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let mut group = HitWorld::new();
    group.add(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    group.add(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    for y in (0..ny).rev() {
        for x in 0..nx {
            let u = (x as f64) / (nx as f64);
            let v = (y as f64) / (ny as f64);
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color = tracing::ray_color(&r, &group);
            let ir = (255.99f64 * color.x) as i32;
            let ig = (255.99f64 * color.y) as i32;
            let ib = (255.99f64 * color.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
