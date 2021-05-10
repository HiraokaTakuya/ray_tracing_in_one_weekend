use crate::{
    hittable::{HitRecord, Hittable},
    material::{Dielectric, Lambertian, Material, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::{Color, Point},
};
use rand::prelude::*;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct HittableList<T>(Vec<T>)
where
    T: Hittable;

impl<T> HittableList<T>
where
    T: Hittable,
{
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.0.clear();
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
}

impl<T> Hittable for HittableList<T>
where
    T: Hittable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for item in self.0.iter() {
            if item.hit(r, t_min, closest_so_far, &mut temp) {
                hit_anything = true;
                closest_so_far = temp.t;
                *rec = temp.clone();
            }
        }
        hit_anything
    }
}

pub fn random_scene(rng: &mut dyn rand::RngCore) -> HittableList<Sphere> {
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut v = vec![Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )];
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::new_random(rng) * Color::new_random(rng);
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new_random_range(rng, 0.5..=1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };
                v.push(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }
    v.push(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    v.push(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    ));
    v.push(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    ));
    HittableList::<Sphere>(v)
}
