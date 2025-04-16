use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(self, scalar: f64) -> Point3 {
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
        let r = Ray::new(p, v);
        assert_eq!(r.at(2.0), Point3::new(2.0, 5.0, 4.0))
    }
}
