use rand::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }
    pub fn new_random(rng: &mut dyn rand::RngCore) -> Self {
        Self {
            e: [rng.gen(), rng.gen(), rng.gen()],
        }
    }
    pub fn new_random_range<R>(rng: &mut dyn rand::RngCore, range: R) -> Self
    where
        R: rand::distributions::uniform::SampleRange<f64> + Clone,
    {
        Self {
            e: [
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
                rng.gen_range(range),
            ],
        }
    }
    pub fn new_random_in_unit_sphere(rng: &mut dyn rand::RngCore) -> Self {
        loop {
            let p = Vec3::new_random_range(rng, -1.0..1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn new_random_unit(rng: &mut dyn rand::RngCore) -> Self {
        Self::new_random_in_unit_sphere(rng).unit()
    }
    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn dot(&self, rhs: &Self) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
    pub fn cross(&self, rhs: &Self) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
    pub fn unit(self) -> Self {
        let len = self.length();
        self / len
    }
    pub fn to_string(&self, samples_per_pixel: f64) -> String {
        let scale = 1.0 / samples_per_pixel;
        format!(
            "{} {} {}",
            (256.0 * (self[0] * scale).sqrt().clamp(0.0, 0.999)) as i64,
            (256.0 * (self[1] * scale).sqrt().clamp(0.0, 0.999)) as i64,
            (256.0 * (self[2] * scale).sqrt().clamp(0.0, 0.999)) as i64,
        )
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

//impl std::fmt::Display for Vec3 {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(
//            f,
//            "{} {} {}",
//            (255.999 * self[0]) as i32,
//            (255.999 * self[1]) as i32,
//            (255.999 * self[2]) as i32
//        )
//    }
//}

#[allow(dead_code)]
pub type Point = Vec3;
#[allow(dead_code)]
pub type Direction = Vec3;
#[allow(dead_code)]
pub type Color = Vec3;

#[test]
fn test_length() {
    let e = [10.0, 20.0, 30.0];
    let v = Vec3::new(e[0], e[1], e[2]);
    let ls = v.length_squared();
    let ls_ans = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]) as i32;
    assert_eq!(ls_ans, 1400);
    assert_eq!(ls.round() as i32, ls_ans);
    let l = v.length();
    let l_ans: i32 = f64::from(ls_ans).sqrt().round() as i32;
    assert_eq!(l_ans, 37);
    assert_eq!(l.round() as i32, l_ans);
}

#[test]
fn test_neg() {
    let l = Vec3::new(10.0, 20.0, 30.0);
    let v = -l;
    assert_eq!(v[0].round() as i64, -10);
    assert_eq!(v[1].round() as i64, -20);
    assert_eq!(v[2].round() as i64, -30);
    let v = -v;
    assert_eq!(v[0].round() as i64, 10);
    assert_eq!(v[1].round() as i64, 20);
    assert_eq!(v[2].round() as i64, 30);
}

#[test]
fn test_add() {
    let l = Vec3::new(10.0, 20.0, 30.0);
    let r = Vec3::new(10.0, 20.0, 30.0);
    let v = l + r;
    assert_eq!(v[0].round() as i64, 20);
    assert_eq!(v[1].round() as i64, 40);
    assert_eq!(v[2].round() as i64, 60);
}

#[test]
fn test_add_assign() {
    let mut l = Vec3::new(10.0, 20.0, 30.0);
    let r = Vec3::new(10.0, 20.0, 30.0);
    l += r;
    assert_eq!(l[0].round() as i64, 20);
    assert_eq!(l[1].round() as i64, 40);
    assert_eq!(l[2].round() as i64, 60);
}

#[test]
fn test_sub() {
    let l = Vec3::new(10.0, 20.0, 30.0);
    let r = Vec3::new(5.0, 10.0, 15.0);
    let v = l - r;
    assert_eq!(v[0].round() as i64, 5);
    assert_eq!(v[1].round() as i64, 10);
    assert_eq!(v[2].round() as i64, 15);
}

#[test]
fn test_sub_assign() {
    let mut l = Vec3::new(10.0, 20.0, 30.0);
    let r = Vec3::new(5.0, 10.0, 15.0);
    l -= r;
    assert_eq!(l[0].round() as i64, 5);
    assert_eq!(l[1].round() as i64, 10);
    assert_eq!(l[2].round() as i64, 15);
}

#[test]
fn test_mul() {
    let l = Vec3::new(10.0, 20.0, 30.0);
    let r = 3.0;
    let v = l * r;
    assert_eq!(v[0].round() as i64, 30);
    assert_eq!(v[1].round() as i64, 60);
    assert_eq!(v[2].round() as i64, 90);
}

#[test]
fn test_mul_assign() {
    let mut l = Vec3::new(10.0, 20.0, 30.0);
    let r = 3.0;
    l *= r;
    assert_eq!(l[0].round() as i64, 30);
    assert_eq!(l[1].round() as i64, 60);
    assert_eq!(l[2].round() as i64, 90);
}

#[test]
fn test_div() {
    let l = Vec3::new(10.0, 20.0, 30.0);
    let r = 2.0;
    let v = l / r;
    assert_eq!(v[0].round() as i64, 5);
    assert_eq!(v[1].round() as i64, 10);
    assert_eq!(v[2].round() as i64, 15);
}

#[test]
fn test_div_assign() {
    let mut l = Vec3::new(10.0, 20.0, 30.0);
    let r = 2.0;
    l /= r;
    assert_eq!(l[0].round() as i64, 5);
    assert_eq!(l[1].round() as i64, 10);
    assert_eq!(l[2].round() as i64, 15);
}

#[test]
fn test_index() {
    let l = Vec3::new(10.0, 20.0, 30.0);
    assert_eq!(l[0].round() as i64, 10);
    assert_eq!(l[1].round() as i64, 20);
    assert_eq!(l[2].round() as i64, 30);
}

#[test]
fn test_index_mut() {
    let mut l = Vec3::new(10.0, 20.0, 30.0);
    l[0] = 5.0;
    l[1] = 10.0;
    l[2] = 15.0;
    assert_eq!(l[0].round() as i64, 5);
    assert_eq!(l[1].round() as i64, 10);
    assert_eq!(l[2].round() as i64, 15);
}
