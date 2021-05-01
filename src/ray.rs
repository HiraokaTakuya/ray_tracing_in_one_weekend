use crate::vec3::Direction;
use crate::vec3::{Point, Vec3};
use crate::{
    hittable::{HitRecord, Hittable},
    vec3::Color,
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
}

#[allow(dead_code)]
impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
    pub fn color<T>(&self, world: &T, rng: &mut dyn rand::RngCore, depth: i64) -> Color
    where
        T: Hittable,
    {
        if depth <= 0 {
            return Color::default();
        }
        let mut rec = HitRecord::default();
        if world.hit(&self, 0.001, std::f64::INFINITY, &mut rec) {
            let target = rec.point + Vec3::new_random_in_hemisphere(rng, &rec.normal);
            return 0.5
                * Ray {
                    origin: rec.point,
                    direction: target - rec.point,
                }
                .color(world, rng, depth - 1);
        }
        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

#[allow(dead_code)]
fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

#[allow(dead_code)]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
