use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn red() -> Color {
        Color {
            x: 1.,
            y: 0.,
            z: 0.,
        }
    }
    pub fn green() -> Color {
        Color {
            x: 0.,
            y: 1.,
            z: 0.,
        }
    }
    pub fn blue() -> Color {
        Color {
            x: 0.,
            y: 0.,
            z: 1.,
        }
    }
    pub fn white() -> Color {
        Color {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }
    pub fn black() -> Color {
        Vec3::zero()
    }
}

pub fn write_color(color: Color) {
    let color = color.clamp(0., 0.999);

    let color = color * 256.;

    let Color { x: r, y: g, z: b } = color;
    let (r, g, b) = (r as u8, g as u8, b as u8);
    println!("{r} {g} {b}")
}
