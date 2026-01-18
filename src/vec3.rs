use std::iter::Sum;
use std::ops;
use std::ops::{AddAssign, SubAssign};

pub type Point3 = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

macro_rules! vec3_vec3_add {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Add<$rhs> for $lhs {
            type Output = Vec3;
            fn add(self, rhs: $rhs) -> Vec3 {
                Vec3 {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }
    };
}
vec3_vec3_add!(Vec3, Vec3);
vec3_vec3_add!(Vec3, &Vec3);
vec3_vec3_add!(&Vec3, Vec3);
vec3_vec3_add!(&Vec3, &Vec3);
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Sum for Vec3 {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Vec3::zero(), |a, b| {a + b})
    }
}

macro_rules! vec3_vec3_sub {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Sub<$rhs> for $lhs {
            type Output = Vec3;
            fn sub(self, rhs: $rhs) -> Vec3 {
                Vec3 {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
            }
        }
    };
}
vec3_vec3_sub!(Vec3, Vec3);
vec3_vec3_sub!(Vec3, &Vec3);
vec3_vec3_sub!(&Vec3, Vec3);
vec3_vec3_sub!(&Vec3, &Vec3);
impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

macro_rules! vec3_f64_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Vec3;
            fn mul(self, rhs: $rhs) -> Vec3 {
                Vec3 {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                }
            }
        }
        impl std::ops::Mul<$lhs> for $rhs {
            type Output = Vec3;
            fn mul(self, rhs: $lhs) -> Vec3 {
                Vec3 {
                    x: self * rhs.x,
                    y: self * rhs.y,
                    z: self * rhs.z,
                }
            }
        }
    };
}
vec3_f64_mul!(Vec3, f64);
vec3_f64_mul!(&Vec3, f64);
vec3_f64_mul!(Vec3, &f64);
vec3_f64_mul!(&Vec3, &f64);
macro_rules! vec3_u16_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Vec3;
            fn mul(self, rhs: $rhs) -> Vec3 {
                Vec3 {
                    x: self.x * (rhs as f64),
                    y: self.y * (rhs as f64),
                    z: self.z * (rhs as f64),
                }
            }
        }
        impl std::ops::Mul<$lhs> for $rhs {
            type Output = Vec3;
            fn mul(self, rhs: $lhs) -> Vec3 {
                Vec3 {
                    x: (self as f64) * rhs.x,
                    y: (self as f64) * rhs.y,
                    z: (self as f64) * rhs.z,
                }
            }
        }
    };
}
macro_rules! vec3_u16_borrow_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Vec3;
            fn mul(self, rhs: $rhs) -> Vec3 {
                Vec3 {
                    x: self.x * (*rhs as f64),
                    y: self.y * (*rhs as f64),
                    z: self.z * (*rhs as f64),
                }
            }
        }
        impl std::ops::Mul<$lhs> for $rhs {
            type Output = Vec3;
            fn mul(self, rhs: $lhs) -> Vec3 {
                Vec3 {
                    x: (*self as f64) * rhs.x,
                    y: (*self as f64) * rhs.y,
                    z: (*self as f64) * rhs.z,
                }
            }
        }
    };
}
vec3_u16_mul!(Vec3, u16);
vec3_u16_mul!(&Vec3, u16);
vec3_u16_borrow_mul!(Vec3, &u16);
vec3_u16_borrow_mul!(&Vec3, &u16);
macro_rules! vec3_vec3_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Vec3;
            fn mul(self, rhs: $rhs) -> Vec3 {
                Vec3 {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                    z: self.z * rhs.z,
                }
            }
        }
    };
}
vec3_vec3_mul!(Vec3, Vec3);
vec3_vec3_mul!(&Vec3, Vec3);
vec3_vec3_mul!(Vec3, &Vec3);
vec3_vec3_mul!(&Vec3, &Vec3);
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

macro_rules! vec3_f64_div {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = Vec3;
            fn div(self, rhs: $rhs) -> Vec3 {
                Vec3 {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                }
            }
        }
    };
}
vec3_f64_div!(Vec3, f64);
vec3_f64_div!(Vec3, &f64);
vec3_f64_div!(&Vec3, f64);
vec3_f64_div!(&Vec3, &f64);
macro_rules! vec3_vec3_div {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = Vec3;
            fn div(self, rhs: $rhs) -> Vec3 {
                Vec3 {
                    x: self.x / rhs.x,
                    y: self.y / rhs.y,
                    z: self.z / rhs.z,
                }
            }
        }
    };
}
vec3_vec3_div!(Vec3, Vec3);
vec3_vec3_div!(Vec3, &Vec3);
vec3_vec3_div!(&Vec3, Vec3);
vec3_vec3_div!(&Vec3, &Vec3);
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub const fn zero() -> Vec3 {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub const fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub const fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub const fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn normalize(&self) -> Vec3 {
        self / self.length()
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }
}
