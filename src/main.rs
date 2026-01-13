use crate::vec3::{Point3, Vec3};
use crate::ray::{Ray};
mod vec3;
mod ray;

type Color = Vec3;

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;

    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let image_height = image_height.max(1);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64)  / image_height as f64);
    let camera_center = Point3::zero();

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., viewport_height, 0.);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_top_left_pos = camera_center - Vec3::new(0., 0., focal_length);

    let first_pixel_loc = viewport_top_left_pos + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_center = first_pixel_loc + (pixel_delta_u * x as f64) + (pixel_delta_v * y as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let pixel_color = color_from_ray(ray);
            write_color(pixel_color);
        }
    }
}

fn color_from_ray(ray: Ray) -> Color {
    Color {
        x: 0.,
        y: 0.,
        z: 0.,
    }
}

fn write_color(color: Color) {
    let Color {
        x: r,
        y: g,
        z: b,
    } = color;

    let r = (r*255.999) as u8;
    let g = (g*255.999) as u8;
    let b = (b*255.999) as u8;

    println!("{r} {g} {b}")
}
