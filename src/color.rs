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