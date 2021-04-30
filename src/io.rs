use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point};
use rand::prelude::*;

#[allow(dead_code)]
pub fn process() {
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
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::<Sphere>::default();
    world.push(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = camera.ray(u, v);
                color += ray.color(&world);
            }
            println!("{}", color.to_string(samples_per_pixel as f64));
        }
    }
}
