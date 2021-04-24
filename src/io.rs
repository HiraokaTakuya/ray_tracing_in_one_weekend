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
pub fn process2() {
    // Image
    struct AspectRatio {
        width: usize,
        height: usize,
    }
    let aspect_ratio = AspectRatio {
        width: 16,
        height: 9,
    };
    let image_width = 400;
    let image_height = image_width * aspect_ratio.height / aspect_ratio.width;

    // Camera
    let viewpoint_height = 2.0;
    let viewpoint_width = viewpoint_height * aspect_ratio.width as f64 / aspect_ratio.height as f64;
    let focal_length = 1.0;
    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewpoint_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewpoint_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel = r.color();
            println!("{}", pixel);
        }
    }
}
