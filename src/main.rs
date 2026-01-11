use crate::vec3::Vec3;
mod vec3;
mod ray;

type Color = Vec3;

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;

    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let image_height = if image_height < 1 {1} else {image_height};


    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64)  / image_height as f64);

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for y in 0..image_height {
        for x in 0..image_width {
            let color = Color {
                x: (x as f64) / (image_width-1) as f64,
                y: (y as f64) / (image_height-1) as f64,
                z: 0.,
            };
            write_color(color)
        }
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
