mod geometry;

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);

    for y in (0..ny).rev() {
        for x in 0..nx {
            let r = (x as f32) / (nx as f32);
            let g = (y as f32) / (ny as f32);
            let b = 0.2f32;
            let ir = (255.99f32 * r) as i32;
            let ig = (255.99f32 * g) as i32;
            let ib = (255.99f32 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
