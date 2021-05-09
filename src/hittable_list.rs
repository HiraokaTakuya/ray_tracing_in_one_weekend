use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

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
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
}

impl<T> Hittable for HittableList<T>
where
    T: Hittable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp = HitRecord::new();
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
