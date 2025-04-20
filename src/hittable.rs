use std::sync::Arc;

use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        normal: Vec3,
        material: Arc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> Self {
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, normal: &Vec3) {
        self.front_face = ray.direction().dot(normal) < 0.0;
        self.normal = if self.front_face { *normal } else { -*normal };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Point3::default(),
            normal: Point3::default(),
            material: Arc::new(Lambertian::default()),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let point = Point3::new(0.0, 1.0, 2.0);
        let normal = Vec3::new(1.0, 3.0, -1.0);
        let material = Arc::new(Lambertian::default());
        let t = 2.88;
        let front_face = false;
        let record = HitRecord::new(point, normal, material, t, front_face);
        assert_eq!(record.point, point);
        assert_eq!(record.normal, normal);
        assert_eq!(record.t, t);
        assert_eq!(record.front_face, false);
    }

    #[test]
    fn test_default() {
        let record = HitRecord::default();
        assert_eq!(
            record.point,
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );
        assert_eq!(
            record.normal,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );
        assert_eq!(record.t, 0.0);
        assert_eq!(record.front_face, false);
    }
}
