use rand::{random, Rng};
use rand::distr::StandardUniform;
use rand::rngs::StdRng;
use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::hittable_collection::HittableCollection;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    image_width: u16,        // Rendered image height
    image_height: u16,       // Rendered image height
    center: Point3,          // Camera center
    first_pixel_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3,     // Offset to pixel to the right
    pixel_delta_v: Vec3,     // Offset to pixel below
    msaa_level: u8,          // Count of rows and columns of rays we should cast per pixel
    max_light_bounces: u8,           // Maximum number of ray bounces
}

impl Camera {
    pub fn new(image_width: u16, aspect_ratio: f64, msaa_level: u8, max_light_bounces: u8) -> Camera {
        // image dimensions
        let image_height = ((image_width as f64) / aspect_ratio) as u16;
        let image_height = image_height.max(1);

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
            image_width,
            image_height,
            center: camera_center,
            first_pixel_loc,
            pixel_delta_u,
            pixel_delta_v,
            msaa_level,
            max_light_bounces,
        }
    }

    pub fn render(&self, world: &HittableCollection) {
        let image_width = self.image_width;
        let image_height = self.image_height;
        println!("P3");
        println!("{image_width} {image_height}");
        println!("255");

        let pixel_color_scale = 1. / ((self.msaa_level as f64) * (self.msaa_level as f64));
        for y in 0..image_height {
            for x in 0..image_width {
                let pixel_color: Vec3 = self
                    .rays_from_pixel(x, y, self.msaa_level)
                    .iter()
                    .map(|ray| self.color_from_ray(ray, world, self.max_light_bounces))
                    .sum();

                write_color(pixel_color * pixel_color_scale);
            }
        }
    }

    fn rays_from_pixel(&self, x: u16, y: u16, msaa_level: u8) -> Vec<Ray> {
        let mut points: Vec<(f64, f64)> = Vec::new();
        for y in 1..=msaa_level {
            for x in 1..=msaa_level {
                let x = x as f64;
                let y = y as f64;
                let n = msaa_level as f64;
                points.push((x / n - 1. / (2. * n), y / n - 1. / (2. * n)));
            }
        }
        points
            .iter()
            .map(|(x_offset, y_offset)| {
                let pixel_sample = self.first_pixel_loc
                    + ((x as f64 + x_offset) * self.pixel_delta_u)
                    + ((y as f64 + y_offset) * self.pixel_delta_v);

                let ray_origin = self.center;
                let ray_direction = pixel_sample - ray_origin;
                Ray {
                    origin: ray_origin,
                    direction: ray_direction,
                }
            })
            .collect()
    }

    fn color_from_ray(&self, ray: &Ray, world: &HittableCollection, remaining_bounces: u8) -> Color {
        if remaining_bounces <= 0 {
            return Color::black();
        }
        let hit = world.hit(
            &ray,
            Interval {
                min: 0.001,
                max: f64::INFINITY,
            },
        );
        match hit {
            Some(hit) => {
                let front_face = Vec3::dot(&hit.normal, &ray.direction) < 0.;
                let camera_side_normal = if front_face {
                    hit.normal
                } else {
                    -hit.normal
                };
                let random_unit_vector: Vec3 = rand::rng().sample(StandardUniform);
                let bounce_direction = camera_side_normal + random_unit_vector;
                0.5 * self.color_from_ray(&Ray{origin:hit.p, direction:bounce_direction}, world, remaining_bounces-1)
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
