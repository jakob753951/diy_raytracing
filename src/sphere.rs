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
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        Some(Hit { p, normal, t, })
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = center - ray.origin;
    let a = ray.direction.length_squared();
    let h = Vec3::dot(&ray.direction, &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0. {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}