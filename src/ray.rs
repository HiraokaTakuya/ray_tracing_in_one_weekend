use crate::vec3::Color;
use crate::vec3::Direction;
use crate::vec3::Point;
use crate::vec3::Vec3;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
}

#[allow(dead_code)]
impl Ray {
    fn at(self, t: f64) -> Point {
        self.origin + self.direction * t
    }
    pub fn color(&self) -> Color {
        let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, &self);
        if t > 0.0 {
            let n = (self.clone().at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
            return 0.5 * Color::new(n[0] + 1.0, n[1] + 1.0, n[2] + 1.0);
        }
        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

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
