use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Color, Point};
use rand::prelude::*;
use std::rc::Rc;

#[allow(dead_code)]
pub fn process() {
    // Image
    struct AspectRatio {
        width: usize,
        height: usize,
    }
    impl AspectRatio {
        fn ratio(&self) -> f64 {
            self.width as f64 / self.height as f64
        }
    }
    let aspect_ratio = AspectRatio {
        width: 16,
        height: 9,
    };
    let image_width = 400;
    let image_height = image_width * aspect_ratio.height / aspect_ratio.width;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let r = (std::f64::consts::PI / 4.0).cos();
    let mut world = HittableList::<Sphere>::default();
    let material_left = Lambertian::new(Color::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Color::new(1.0, 0.0, 0.0));

    world.push(Sphere::new(
        Point::new(-r, 0.0, -1.0),
        r,
        Rc::new(material_left),
    ));
    world.push(Sphere::new(
        Point::new(r, 0.0, -1.0),
        r,
        Rc::new(material_right),
    ));

    // Camera
    let camera = Camera::new(90.0, aspect_ratio.ratio());

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
                color += ray.color(&world, &mut rng, max_depth);
            }
            println!("{}", color.to_string(samples_per_pixel as f64));
        }
    }
}
