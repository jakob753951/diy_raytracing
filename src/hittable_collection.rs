use crate::hittable::{Hit, Hittable};
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

impl Hittable for HittableCollection {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        for object in &self.objects {
            match closest_hit {
                None => t_max,
                Some(hit) => hit.t,
            };
            let hit = object.hit(ray, t_min, t_max);
            
            match hit {
                None => continue,
                Some(hit) => {
                    match closest_hit {
                        None => closest_hit = Some(hit),
                        Some(previous_closest_hit) => {
                            if hit.t < previous_closest_hit.t {
                                closest_hit = Some(hit);
                            }
                        }
                    }
                }
            }
        }
        
        closest_hit
    }
}