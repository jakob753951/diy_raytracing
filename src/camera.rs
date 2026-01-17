use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::hittable_collection::HittableCollection;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u16,
    image_height: u16, // Rendered image height
    center: Point3, // Camera center
    first_pixel_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
    pub fn new(image_width: u16, aspect_ratio: f64) -> Camera {
        // image dimensions
        let image_height = ((image_width as f64) / aspect_ratio) as i32;
        let image_height = image_height.max(1) as u16;

        let camera_center = Point3::zero();

        // viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((image_width as f64) / image_height as f64);
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // delta vectors between pixels on x and y
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // position of the top-left corner of the viewport
        let viewport_top_left_pos =
            camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;

        // position of the top-left pixel in the viewport
        // not the same as the corner, due to offset (pixel point should be in the middle of the area it shows)
        let first_pixel_loc = viewport_top_left_pos + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center: camera_center,
            first_pixel_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableCollection) {
        let image_width = self.image_width;
        let image_height = self.image_height;
        println!("P3");
        println!("{image_width} {image_height}");
        println!("255");

        for y in 0..image_height {
            for x in 0..image_width {
                let pixel_center =
                    self.first_pixel_loc + (self.pixel_delta_u * x as f64) + (self.pixel_delta_v * y as f64);
                let ray_direction = pixel_center - self.center;
                let ray = Ray {
                    origin: self.center,
                    direction: ray_direction,
                };

                let pixel_color = self.color_from_ray(ray, &world);
                write_color(pixel_color);
            }
        }
    }

    fn color_from_ray(&self, ray: Ray, world: &HittableCollection) -> Color {
        let hit = world.hit(
            &ray,
            Interval {
                min: 0.,
                max: f64::INFINITY,
            },
        );
        match hit {
            Some(hit) => {
                let front_face = Vec3::dot(&hit.normal, &ray.direction) < 0.;

                0.5 * (hit.normal + Color::white())
            }
            None => {
                let unit_direction = ray.direction.normalize();
                let a = 0.5 * (unit_direction.y + 1.0);
                lerp(
                    a,
                    Color {
                        x: 0.5,
                        y: 0.7,
                        z: 1.0,
                    },
                    Color::white(),
                )
            }
        }
    }
}

fn lerp(factor: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - factor) * end + factor * start
}