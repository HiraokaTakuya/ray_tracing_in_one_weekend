use rand::RngCore;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material: CloneMaterial {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut dyn RngCore,
    ) -> bool;
}

pub trait CloneMaterial {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> CloneMaterial for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut dyn RngCore,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::new_random_unit(rng);

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray {
            origin: rec.point,
            direction: scatter_direction,
        };
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut dyn RngCore,
    ) -> bool {
        let reflected = r_in.direction.unit().reflect(&rec.normal);
        *scattered = Ray {
            origin: rec.point,
            direction: reflected + self.fuzz * Vec3::new_random_in_unit_sphere(rng),
        };
        *attenuation = self.albedo;
        scattered.direction.dot(&rec.normal) > 0.0
    }
}
