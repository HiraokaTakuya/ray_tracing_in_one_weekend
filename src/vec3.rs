use std::marker::PhantomData;

pub struct PointType;
pub struct ColorType;

pub trait PointOrColor {}

impl PointOrColor for PointType {}
impl PointOrColor for ColorType {}

#[derive(Debug, Clone)]
pub struct Vec3<T>
where
    T: PointOrColor,
{
    e: [f64; 3],
    _marker: PhantomData<fn() -> T>,
}

#[allow(dead_code)]
impl<T> Vec3<T>
where
    T: PointOrColor,
{
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            e: [e0, e1, e2],
            _marker: PhantomData,
        }
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
}

impl<T> std::ops::Neg for Vec3<T>
where
    T: PointOrColor,
{
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::<T>::new(-self[0], -self[1], -self[2])
    }
}

impl<T> std::ops::Add for Vec3<T>
where
    T: PointOrColor,
{
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Self::Output {
        Vec3::<T>::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl<T> std::ops::AddAssign for Vec3<T>
where
    T: PointOrColor,
{
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl<T> std::ops::Sub for Vec3<T>
where
    T: PointOrColor,
{
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Self::Output {
        Vec3::<T>::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl<T> std::ops::SubAssign for Vec3<T>
where
    T: PointOrColor,
{
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl<T> std::ops::Mul<f64> for Vec3<T>
where
    T: PointOrColor,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::<T>::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl<T> std::ops::MulAssign<f64> for Vec3<T>
where
    T: PointOrColor,
{
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl<T> std::ops::Div<f64> for Vec3<T>
where
    T: PointOrColor,
{
    type Output = Vec3<T>;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::<T>::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl<T> std::ops::DivAssign<f64> for Vec3<T>
where
    T: PointOrColor,
{
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl<T> std::ops::Index<usize> for Vec3<T>
where
    T: PointOrColor,
{
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Vec3<T>
where
    T: PointOrColor,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

#[allow(dead_code)]
pub type Point = Vec3<PointType>;
#[allow(dead_code)]
pub type Color = Vec3<ColorType>;

#[test]
fn test_length() {
    fn f<T: PointOrColor>() {
        let e = [10.0, 20.0, 30.0];
        let v = Vec3::<T>::new(e[0], e[1], e[2]);
        let ls = v.length_squared();
        let ls_ans = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]) as i32;
        assert_eq!(ls_ans, 1400);
        assert_eq!(ls.round() as i32, ls_ans);
        let l = v.length();
        let l_ans: i32 = f64::from(ls_ans).sqrt().round() as i32;
        assert_eq!(l_ans, 37);
        assert_eq!(l.round() as i32, l_ans);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_neg() {
    fn f<T: PointOrColor>() {
        let l = Vec3::<T>::new(10.0, 20.0, 30.0);
        let v = -l;
        assert_eq!(v[0].round() as i64, -10);
        assert_eq!(v[1].round() as i64, -20);
        assert_eq!(v[2].round() as i64, -30);
        let v = -v;
        assert_eq!(v[0].round() as i64, 10);
        assert_eq!(v[1].round() as i64, 20);
        assert_eq!(v[2].round() as i64, 30);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_add() {
    fn f<T: PointOrColor>() {
        let l = Vec3::<T>::new(10.0, 20.0, 30.0);
        let r = Vec3::<T>::new(10.0, 20.0, 30.0);
        let v = l + r;
        assert_eq!(v[0].round() as i64, 20);
        assert_eq!(v[1].round() as i64, 40);
        assert_eq!(v[2].round() as i64, 60);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_add_assign() {
    fn f<T: PointOrColor>() {
        let mut l = Vec3::<T>::new(10.0, 20.0, 30.0);
        let r = Vec3::<T>::new(10.0, 20.0, 30.0);
        l += r;
        assert_eq!(l[0].round() as i64, 20);
        assert_eq!(l[1].round() as i64, 40);
        assert_eq!(l[2].round() as i64, 60);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_sub() {
    fn f<T: PointOrColor>() {
        let l = Vec3::<T>::new(10.0, 20.0, 30.0);
        let r = Vec3::<T>::new(5.0, 10.0, 15.0);
        let v = l - r;
        assert_eq!(v[0].round() as i64, 5);
        assert_eq!(v[1].round() as i64, 10);
        assert_eq!(v[2].round() as i64, 15);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_sub_assign() {
    fn f<T: PointOrColor>() {
        let mut l = Vec3::<T>::new(10.0, 20.0, 30.0);
        let r = Vec3::<T>::new(5.0, 10.0, 15.0);
        l -= r;
        assert_eq!(l[0].round() as i64, 5);
        assert_eq!(l[1].round() as i64, 10);
        assert_eq!(l[2].round() as i64, 15);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_mul() {
    fn f<T: PointOrColor>() {
        let l = Vec3::<T>::new(10.0, 20.0, 30.0);
        let r = 3.0;
        let v = l * r;
        assert_eq!(v[0].round() as i64, 30);
        assert_eq!(v[1].round() as i64, 60);
        assert_eq!(v[2].round() as i64, 90);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_mul_assign() {
    fn f<T: PointOrColor>() {
        let mut l = Vec3::<T>::new(10.0, 20.0, 30.0);
        let r = 3.0;
        l *= r;
        assert_eq!(l[0].round() as i64, 30);
        assert_eq!(l[1].round() as i64, 60);
        assert_eq!(l[2].round() as i64, 90);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_div() {
    fn f<T: PointOrColor>() {
        let l = Vec3::<T>::new(10.0, 20.0, 30.0);
        let r = 2.0;
        let v = l / r;
        assert_eq!(v[0].round() as i64, 5);
        assert_eq!(v[1].round() as i64, 10);
        assert_eq!(v[2].round() as i64, 15);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_div_assign() {
    fn f<T: PointOrColor>() {
        let mut l = Vec3::<T>::new(10.0, 20.0, 30.0);
        let r = 2.0;
        l /= r;
        assert_eq!(l[0].round() as i64, 5);
        assert_eq!(l[1].round() as i64, 10);
        assert_eq!(l[2].round() as i64, 15);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_index() {
    fn f<T: PointOrColor>() {
        let l = Vec3::<T>::new(10.0, 20.0, 30.0);
        assert_eq!(l[0].round() as i64, 10);
        assert_eq!(l[1].round() as i64, 20);
        assert_eq!(l[2].round() as i64, 30);
    }
    f::<PointType>();
    f::<ColorType>();
}

#[test]
fn test_index_mut() {
    fn f<T: PointOrColor>() {
        let mut l = Vec3::<T>::new(10.0, 20.0, 30.0);
        l[0] = 5.0;
        l[1] = 10.0;
        l[2] = 15.0;
        assert_eq!(l[0].round() as i64, 5);
        assert_eq!(l[1].round() as i64, 10);
        assert_eq!(l[2].round() as i64, 15);
    }
    f::<PointType>();
    f::<ColorType>();
}
