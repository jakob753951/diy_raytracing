use rand::distr::StandardUniform;
use rand::{rng, Rng};
use crate::color::Color;
use crate::hittable::Hit;
use crate::material::{Material, Scattering};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, hit: Hit) -> Option<Scattering> {
        let scatter_direction = hit.normal + rng().sample::<Vec3, _>(StandardUniform);
        Some(Scattering {
            scattered: Ray {
                origin: hit.location,
                direction: scatter_direction,
            },
            attenuation: self.albedo,
        })
    }
}