use crate::color::Color;
use crate::hittable::Hit;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scattering>;
}

pub struct Scattering {
    pub(crate) attenuation: Color,
    pub(crate) scattered: Ray,
}
