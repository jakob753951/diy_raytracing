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
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scattering> {
        let front_face = Vec3::dot(&hit.normal, &ray.direction) < 0.;
        let camera_side_normal = if front_face {
            hit.normal
        } else {
            -hit.normal
        };

        let mut scatter_direction = camera_side_normal + rng().sample::<Vec3, _>(StandardUniform);

        if scatter_direction.length() < 1e-8 {
            scatter_direction = camera_side_normal;
        }

        Some(Scattering {
            scattered: Ray {
                origin: hit.location,
                direction: scatter_direction,
            },
            attenuation: self.albedo,
        })
    }
}