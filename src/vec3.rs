#[derive(Debug, Clone)]
pub struct Vec3(f64, f64, f64);

#[allow(dead_code)]
impl Vec3 {
    fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

#[test]
fn test_length() {
    let v = Vec3(10.0, 20.0, 30.0);
    let ls = v.length_squared();
    let ls_ans = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2) as i32;
    assert_eq!(ls_ans, 1400);
    assert_eq!(ls.round() as i32, ls_ans);
    let l = v.length();
    let l_ans: i32 = f64::from(ls_ans).sqrt().round() as i32;
    assert_eq!(l_ans, 37);
    assert_eq!(l.round() as i32, l_ans);
}

#[test]
fn test_neg() {
    let l = Vec3(10.0, 20.0, 30.0);
    let v = -l;
    assert_eq!(v.0.round() as i64, -10);
    assert_eq!(v.1.round() as i64, -20);
    assert_eq!(v.2.round() as i64, -30);
    let v = -v;
    assert_eq!(v.0.round() as i64, 10);
    assert_eq!(v.1.round() as i64, 20);
    assert_eq!(v.2.round() as i64, 30);
}

#[test]
fn test_add() {
    let l = Vec3(10.0, 20.0, 30.0);
    let r = Vec3(10.0, 20.0, 30.0);
    let v = l + r;
    assert_eq!(v.0.round() as i64, 20);
    assert_eq!(v.1.round() as i64, 40);
    assert_eq!(v.2.round() as i64, 60);
}

#[test]
fn test_add_assign() {
    let mut l = Vec3(10.0, 20.0, 30.0);
    let r = Vec3(10.0, 20.0, 30.0);
    l += r;
    assert_eq!(l.0.round() as i64, 20);
    assert_eq!(l.1.round() as i64, 40);
    assert_eq!(l.2.round() as i64, 60);
}

#[test]
fn test_sub() {
    let l = Vec3(10.0, 20.0, 30.0);
    let r = Vec3(5.0, 10.0, 15.0);
    let v = l - r;
    assert_eq!(v.0.round() as i64, 5);
    assert_eq!(v.1.round() as i64, 10);
    assert_eq!(v.2.round() as i64, 15);
}

#[test]
fn test_sub_assign() {
    let mut l = Vec3(10.0, 20.0, 30.0);
    let r = Vec3(5.0, 10.0, 15.0);
    l -= r;
    assert_eq!(l.0.round() as i64, 5);
    assert_eq!(l.1.round() as i64, 10);
    assert_eq!(l.2.round() as i64, 15);
}

#[test]
fn test_mul() {
    let l = Vec3(10.0, 20.0, 30.0);
    let r = 3.0;
    let v = l * r;
    assert_eq!(v.0.round() as i64, 30);
    assert_eq!(v.1.round() as i64, 60);
    assert_eq!(v.2.round() as i64, 90);
}

#[test]
fn test_mul_assign() {
    let mut l = Vec3(10.0, 20.0, 30.0);
    let r = 3.0;
    l *= r;
    assert_eq!(l.0.round() as i64, 30);
    assert_eq!(l.1.round() as i64, 60);
    assert_eq!(l.2.round() as i64, 90);
}

#[test]
fn test_div() {
    let l = Vec3(10.0, 20.0, 30.0);
    let r = 2.0;
    let v = l / r;
    assert_eq!(v.0.round() as i64, 5);
    assert_eq!(v.1.round() as i64, 10);
    assert_eq!(v.2.round() as i64, 15);
}

#[test]
fn test_div_assign() {
    let mut l = Vec3(10.0, 20.0, 30.0);
    let r = 2.0;
    l /= r;
    assert_eq!(l.0.round() as i64, 5);
    assert_eq!(l.1.round() as i64, 10);
    assert_eq!(l.2.round() as i64, 15);
}

#[derive(Debug, Clone)]
struct Point(pub Vec3);

#[allow(dead_code)]
impl Point {
    pub fn length_squared(&self) -> f64 {
        self.0.length_squared()
    }
    pub fn length(&self) -> f64 {
        self.0.length()
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point(-self.0)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point(self.0 + other.0)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        Point(self.0 - other.0)
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point(self.0 * rhs)
    }
}

impl std::ops::MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point(self.0 / rhs)
    }
}

impl std::ops::DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
    }
}

#[derive(Debug, Clone)]
pub struct Color(pub Vec3);

#[allow(dead_code)]
impl Color {
    pub fn length_squared(&self) -> f64 {
        self.0.length_squared()
    }
    pub fn length(&self) -> f64 {
        self.0.length()
    }
}

impl std::ops::Neg for Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        Color(-self.0)
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color(self.0 + other.0)
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        Color(self.0 - other.0)
    }
}

impl std::ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs)
    }
}

impl std::ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color(self.0 / rhs)
    }
}

impl std::ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
    }
}
