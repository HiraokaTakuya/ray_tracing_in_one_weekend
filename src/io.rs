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
    let aspect_ratio = AspectRatio {
        width: 16,
        height: 9,
    };
    let image_width = 400;
    let image_height = image_width * aspect_ratio.height / aspect_ratio.width;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::<Sphere>::default();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Dielectric::new(1.5);
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.push(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground),
    ));
    world.push(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(material_center),
    ));
    world.push(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    ));
    world.push(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    ));

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
                color += ray.color(&world, &mut rng, max_depth);
            }
            println!("{}", color.to_string(samples_per_pixel as f64));
        }
    }
}
