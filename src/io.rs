use crate::{
    camera::Camera,
    hittable_list::random_scene,
    vec3::{Color, Point, Vec3},
};
use rand::prelude::*;

pub fn process() {
    let mut rng = rand::thread_rng();

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
        width: 3,
        height: 2,
    };
    let image_width = 1200;
    let image_height = image_width * aspect_ratio.height / aspect_ratio.width;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene(&mut rng);

    // Camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
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
