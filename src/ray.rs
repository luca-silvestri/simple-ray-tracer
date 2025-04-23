use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, scalar: f64) -> Point3 {
        self.origin + scalar * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let p = Point3::new(2.0, 1.0, 0.0);
        let v = Vec3::new(0.0, 2.0, 2.0);
        let t = 0.3;
        let r = Ray::new(p, v, t);
        assert_eq!(r.at(2.0), Point3::new(2.0, 5.0, 4.0))
    }
}
