use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

use rand::Rng;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::rng();
        Vec3 {
            x: rng.random_range(0.0..1.0),
            y: rng.random_range(0.0..1.0),
            z: rng.random_range(0.0..1.0),
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let candidate = Vec3::random();
            let length = candidate.length_squared();
            if 1e-160 < length && length <= 1.0 {
                return candidate / length.sqrt();
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::rng();
        loop {
            let candidate = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                0.0,
            );
            if candidate.length() < 1.0 {
                return candidate;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere: Vec3 = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
        *incident - 2.0 * incident.dot(&normal) * *normal
    }

    pub fn refract(incident: &Vec3, normal: &Vec3, eta_ratio: f64) -> Vec3 {
        let cos_theta = (-*incident).dot(&normal).min(1.0);
        let ray_out_perpendicular = eta_ratio * (*incident + *normal * cos_theta);
        let ray_out_parallel =
            *normal * -(1.0 - ray_out_perpendicular.length_squared()).abs().sqrt();
        ray_out_perpendicular + ray_out_parallel
    }
}

// Operator overloading
//

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Vec3 {
        iter.fold(Vec3::new(0.0, 0.0, 0.0), |acc, p| {
            Vec3::new(acc.x + p.x, acc.y + p.y, acc.z + p.z)
        })
    }
}

pub type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_create() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.length_squared(), 25.0);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_addition() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_subtraction() {
        let v1 = Vec3::new(5.0, 7.0, 9.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_multiplication() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = v * 2.0;
        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_division() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let result = v / 2.0;
        assert_eq!(result, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_negation() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let result = -v;
        assert_eq!(result, Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let unit_v = v.unit_vector();
        assert!((unit_v.length() - 1.0).abs() < 1e-6);
    }
}
