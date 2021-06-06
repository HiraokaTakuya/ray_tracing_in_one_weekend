use crate::{
    camera::Camera,
    hittable_list::random_scene,
    vec3::{Color, Point, Vec3},
};
use rand::{Rng, SeedableRng};

pub fn process() -> Vec<u8> {
    let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(123);

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
    let image_width = 300;
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
    let mut v: Vec<u8> = vec![];

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = camera.ray(u, v, &mut rng);
                color += ray.color(&world, &mut rng, max_depth);
            }
            let scale = 1.0 / samples_per_pixel as f64;
            v.push((256.0 * (color[0] * scale).sqrt().clamp(0.0, 0.999)) as u8);
            v.push((256.0 * (color[1] * scale).sqrt().clamp(0.0, 0.999)) as u8);
            v.push((256.0 * (color[2] * scale).sqrt().clamp(0.0, 0.999)) as u8);
            v.push(255);
        }
    }
    v
}
