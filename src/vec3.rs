use crate::util::{clamp, random_in_range};
use rand::random;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

// Type aliases for convenience
pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn new_init(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn random() -> Self {
        Self {
            e: [random(), random(), random()],
        }
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        Self {
            e: [
                random_in_range(min, max),
                random_in_range(min, max),
                random_in_range(min, max),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut v: Self;
        loop {
            v = Self::random_in_range(-1.0, 1.0);
            if v.len_squared() < 1.0 {
                break;
            }
        }
        v
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut v: Self;
        loop {
            v = Self::new_init(random_in_range(-1.0, 1.0), random_in_range(-1.0, 1.0), 0.0);
            if v.len_squared() < 1.0 {
                break;
            }
        }
        v
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn len_squared(&self) -> f64 {
        self.e[0].powf(2.0) + self.e[1].powf(2.0) + self.e[2].powf(2.0)
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new_init(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - *n * self.dot(n) * 2.0
    }

    pub fn refract(&self, normal: &Vec3, eta_ratio: f64) -> Vec3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let out_perp = (*self + *normal * cos_theta) * eta_ratio;
        let out_par = *normal * -((1.0 - out_perp.len_squared()).abs() as f64).sqrt();
        out_perp + out_par
    }

    pub fn as_multisample_color_str(&self, samples_per_pixel: u32) -> String {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // scale to a single pixel
        let scale = 1.0 / (samples_per_pixel as f64);
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();
        format!(
            "{} {} {}\n",
            (256.0 * clamp(r, 0.0, 0.999)) as u8,
            (256.0 * clamp(g, 0.0, 0.999)) as u8,
            (256.0 * clamp(b, 0.0, 0.999)) as u8
        )
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.e)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Self {
            e: [self.e[0] * rhs[0], self.e[1] * rhs[1], self.e[2] * rhs[2]],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl Div<f64> for Vec3 {
    // The division of rational numbers is a closed operation.
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        };
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3::new();
        assert_eq!(v, Vec3 { e: [0.0, 0.0, 0.0] });
    }

    #[test]
    fn test_new_init() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3 { e: [1.0, 2.0, 3.0] });
        let v = Color::new_init(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3 { e: [1.0, 2.0, 3.0] });
        let v = Point3::new_init(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3 { e: [1.0, 2.0, 3.0] });
    }

    #[test]
    fn test_accessors() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_mul() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        assert_eq!(v * 3.0, Vec3 { e: [3.0, 6.0, 9.0] });
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3::new_init(1.0, 2.0, 3.0);
        v *= 12.0;
        assert_eq!(
            v,
            Vec3 {
                e: [12.0, 24.0, 36.0]
            }
        );
    }

    #[test]
    fn test_add() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        let v2 = Vec3::new_init(2.0, 3.0, 4.0);
        assert_eq!(v + v2, Vec3 { e: [3.0, 5.0, 7.0] });
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec3::new_init(1.0, 2.0, 3.0);
        let v2 = Vec3::new_init(2.0, 3.0, 4.0);
        v += v2;
        assert_eq!(v, Vec3 { e: [3.0, 5.0, 7.0] });
    }

    #[test]
    fn test_sub() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        let v2 = Vec3::new_init(2.0, 3.0, 2.0);
        assert_eq!(
            v - v2,
            Vec3 {
                e: [-1.0, -1.0, 1.0]
            }
        );
    }

    #[test]
    fn test_div() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        assert_eq!(v / 2.0, Vec3 { e: [0.5, 1.0, 1.5] });
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new_init(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v, Vec3 { e: [0.5, 1.0, 1.5] });
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        assert_eq!(
            -v,
            Vec3 {
                e: [-1.0, -2.0, -3.0]
            }
        );
    }

    #[test]
    fn test_index() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut v = Vec3::new_init(1.0, 2.0, 3.0);
        v[2] = 42.0;
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 42.0);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        assert_eq!(v.len_squared(), 1.0 + 4.0 + 9.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        assert_eq!(v.len(), 14.0f64.sqrt());
    }

    #[test]
    fn test_dot() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        let v2 = Vec3::new_init(2.0, 2.0, 2.0);
        assert_eq!(v.dot(&v2), 2.0 + 4.0 + 6.0);
    }

    #[test]
    fn test_cross() {
        let v = Vec3::new_init(1.0, 2.0, 3.0);
        let v2 = Vec3::new_init(3.0, 2.0, 8.0);
        assert_eq!(
            v.cross(&v2),
            Vec3::new_init(16.0 - 6.0, 9.0 - 8.0, 2.0 - 6.0)
        );
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new_init(0.0, 3.0, 4.0);
        assert_eq!(v.unit_vector(), Vec3::new_init(0.0, 3.0 / 5.0, 4.0 / 5.0));
    }

    #[test]
    fn test_as_color_str() {
        let v = Vec3::new_init(1.0, 1.0, 1.0);
        assert_eq!(v.as_color_str(), "255.999 255.999 255.999\n");
    }
}
