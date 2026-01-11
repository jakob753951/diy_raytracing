fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for y in 0..image_height {
        for x in 0..image_width {
            let r: f64 = (x as f64) / (image_width-1) as f64;
            let g: f64 = (y as f64) / (image_height-1) as f64;
            let b: f64 = 0.;

            let r: u8 = (255.999 * r) as u8;
            let g: u8 = (255.999 * g) as u8;
            let b: u8 = (255.999 * b) as u8;

            println!("{r} {g} {b}")
        }
    }
}
