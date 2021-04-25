use crate::vec3::Color;
use crate::vec3::Direction;
use crate::vec3::Point;

#[allow(dead_code)]
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
        if hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, &self) {
            return Color::new(1.0, 0.0, 0.0);
        }
        let unit_direction = self.direction.unit();
        let t = (unit_direction[1] + 1.0) * 0.5;
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - *center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
