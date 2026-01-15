use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone)]
pub struct Hit {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}