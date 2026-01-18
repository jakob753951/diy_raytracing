use crate::hittable::{Hit, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableCollection {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableCollection {
    pub fn new() -> HittableCollection {
        HittableCollection { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl From<Vec<Box<dyn Hittable>>> for HittableCollection {
    fn from(value: Vec<Box<dyn Hittable>>) -> Self {
        HittableCollection { objects: value }
    }
}

impl Hittable for HittableCollection {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        for object in &self.objects {
            let max_distance_to_search = match closest_hit {
                None => t_interval.max,
                Some(hit) => hit.t,
            };
            let hit = object.hit(ray, Interval::new(t_interval.min, max_distance_to_search));

            match hit {
                None => continue,
                Some(hit) => match closest_hit {
                    None => closest_hit = Some(hit),
                    Some(previous_closest_hit) => {
                        if hit.t < previous_closest_hit.t {
                            closest_hit = Some(hit);
                        }
                    }
                },
            }
        }

        closest_hit
    }
}
