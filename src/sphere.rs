use crate::hittable::{Hit, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<Hit> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = Vec3::dot(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        let mut root = (h - discriminant.sqrt()) / a;
        if !t_interval.surrounds(root) {
            root = (h + discriminant.sqrt()) / a;
            if !t_interval.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let location = ray.at(t);
        let normal = (location - self.center) / self.radius;
        Some(Hit { location, normal, t })
    }
}
