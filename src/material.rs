use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};
use rand::{Rng, RngCore};

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

#[derive(Clone)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut dyn RngCore,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = r_in.direction.unit();
        let cos_theta = {
            let tmp = (-unit_direction).dot(&rec.normal);
            if tmp < 1.0 {
                tmp
            } else {
                1.0
            }
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng.gen() {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal, refraction_ratio)
            };

        *scattered = Ray {
            origin: rec.point,
            direction,
        };
        true
    }
}
