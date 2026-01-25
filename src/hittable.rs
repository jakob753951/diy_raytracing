use std::sync::Arc;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Hit {
    pub location: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<Hit>;
}
