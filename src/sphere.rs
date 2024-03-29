use crate::{
    hittable::{HitRecord, Hittable},
    material::{Lambertian, Material},
    ray::Ray,
    vec3::{Color, Point},
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.material.clone();

        true
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(
            Point::default(),
            f64::default(),
            Rc::new(Lambertian::new(Color::default())),
        )
    }
}
