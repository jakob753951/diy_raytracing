use rand::distr::StandardUniform;
use rand::Rng;
use crate::color::Color;
use crate::hittable::Hit;
use crate::material::{Material, Scattering};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    pub(crate) albedo: Color,
    pub(crate) fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scattering> {

        let front_face = Vec3::dot(&hit.normal, &ray.direction) < 0.;
        let camera_side_normal = if front_face {
            hit.normal
        } else {
            -hit.normal
        };
        let mut reflected = ray.direction.reflect(camera_side_normal);
        reflected = reflected.normalize() + (self.fuzz * rand::rng().sample::<Vec3, _>(StandardUniform));

        if reflected.length() < 1e-8 {
            reflected = camera_side_normal;
        }
        let scattered = Ray { origin: hit.location, direction: reflected };
        if Vec3::dot(&scattered.direction, &hit.normal) > 0. {
            Some(Scattering {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}