use crate::ray::Ray;
use crate::vec3::{Color, Point, Vec3};

#[allow(dead_code)]
pub fn process() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;
    const MAX_BRIGHTNESS: usize = 255;

    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_BRIGHTNESS);
    for h in (0..IMAGE_HEIGHT).rev() {
        for w in 0..IMAGE_WIDTH {
            let color = Color::new(
                w as f64 / (IMAGE_WIDTH - 1) as f64,
                h as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );
            println!("{}", color);
        }
    }
}

#[allow(dead_code)]
pub fn process2() {}
