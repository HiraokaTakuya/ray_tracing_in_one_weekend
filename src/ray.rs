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
        let unit_direction = self.direction.clone().unit();
        let t = (unit_direction[1] + 1.0) * 0.5;
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
