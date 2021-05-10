use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Color, Point, Vec3};
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
    let mut world = HittableList::<Sphere>::default();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

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
        Rc::new(material_left.clone()),
    ));
    world.push(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.45,
        Rc::new(material_left),
    ));
    world.push(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    ));

    // Camera
    let lookfrom = Point::new(3.0, 3.0, 2.0);
    let lookat = Point::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio.ratio(),
        aperture,
        dist_to_focus,
    );

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = camera.ray(u, v, &mut rng);
                color += ray.color(&world, &mut rng, max_depth);
            }
            println!("{}", color.to_string(samples_per_pixel as f64));
        }
    }
}
