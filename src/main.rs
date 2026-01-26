use std::sync::Arc;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_collection::HittableCollection;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
mod camera;
mod color;
mod hittable;
mod hittable_collection;
mod interval;
mod ray;
mod sphere;
mod vec3;
mod material;
mod materials;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 1000;

    let camera = Camera::new(image_width, aspect_ratio, 4, 50);

    // world
    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.,
            material: Arc::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0) }),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Arc::new(Lambertian { albedo: Color::new(0.1, 0.2, 0.5) }),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Arc::new(Metal { albedo: Color::new(0.8, 0.8, 0.8) }),
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Arc::new(Metal { albedo: Color::new(0.8, 0.8, 0.0) }),
        }),
    ];
    let world = HittableCollection::from(objects);

    camera.render(&world);
}
