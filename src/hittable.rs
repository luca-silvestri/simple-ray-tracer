use std::sync::Arc;

use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        normal: Vec3,
        material: Arc<dyn Material>,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
    ) -> Self {
        HitRecord {
            point,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, normal: &Vec3) {
        self.front_face = ray.direction().dot(normal) < 0.0;
        self.normal = if self.front_face { *normal } else { -*normal };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &AABB;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::Lambertian;

    #[test]
    fn test_create() {
        let point = Point3::new(0.0, 1.0, 2.0);
        let normal = Vec3::new(1.0, 3.0, -1.0);
        let material = Arc::new(Lambertian::from_albedo(Color::new(0.5, 0.5, 0.5)));
        let t = 2.88;
        let front_face = false;
        let record = HitRecord::new(point, normal, material, t, 1.0, 2.0, front_face);
        assert_eq!(record.point, point);
        assert_eq!(record.normal, normal);
        assert_eq!(record.t, t);
        assert_eq!(record.front_face, false);
    }
}
