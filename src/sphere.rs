use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius: f64::max(radius, 0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.center - *ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let mut record = HitRecord::default();
        record.t = root;
        record.point = ray.at(record.t);
        let normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &normal);
        record.material = Arc::clone(&self.material);

        return Some(record);
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    use crate::material::Lambertian;

    use super::*;

    #[test]
    fn test_create() {
        let center = Point3::new(0.0, 2.0, 1.0);
        let radius = 3.0;
        let material = Lambertian::default();
        let sphere = Sphere::new(center, radius, Arc::new(material));
        assert_eq!(sphere.center, center);
        assert_eq!(sphere.radius, radius);
    }

    #[test]
    fn test_create_with_negative_radius() {
        let center = Point3::new(0.0, 2.0, 1.0);
        let radius = -3.0;
        let material = Lambertian::default();
        let sphere = Sphere::new(center, radius, Arc::new(material));
        assert_eq!(sphere.center, center);
        assert_eq!(sphere.radius, 0.0);
    }

    #[test]
    fn test_ray_hits_sphere() {
        let sphere = Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::default()),
        );

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.001, f64::INFINITY);
        let result = sphere.hit(&ray, interval);
        assert_eq!(result.is_some(), true, "Ray should hit the sphere");
        let record = result.unwrap();
        assert!(
            (record.t - 0.5).abs() < 1e-6,
            "Expected t ~ 0.5, got {}",
            record.t
        );
        assert!(
            (record.point - Vec3::new(0.0, 0.0, -0.5)).length() < 1e-6,
            "Unexpected hit point: {:?}",
            record.point
        );
        assert!(
            (record.normal - Vec3::new(0.0, 0.0, 1.0)).length() < 1e-6,
            "Unexpected normal: {:?}",
            record.normal
        );
    }

    #[test]
    fn test_ray_misses_sphere() {
        let sphere = Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::default()),
        );

        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.001, f64::INFINITY);
        let result = sphere.hit(&ray, interval);
        assert_eq!(result.is_none(), true, "Ray should miss the sphere");
    }
}
