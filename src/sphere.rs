use std::f64::consts::PI;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn stationary(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center: Ray::new(center, Vec3::new(0.0, 0.0, 0.0), 0.0),
            radius: f64::max(radius, 0.0),
            material,
            bbox: AABB::from_extremes(&(center - rvec), &(center + rvec)),
        }
    }

    pub fn moving(start: Point3, end: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::from_extremes(&(start - rvec), &(start + rvec));
        let box2 = AABB::from_extremes(&(end - rvec), &(end + rvec));
        Sphere {
            center: Ray::new(start, end - start, 0.0),
            radius: f64::max(radius, 0.0),
            material,
            bbox: box1.union(&box2),
        }
    }

    fn get_sphere_uv(&self, point: &Point3) -> (f64, f64) {
        let theta = f64::acos(-point.y);
        let phi = f64::atan2(-point.z, point.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time());
        let oc: Vec3 = current_center - *ray.origin();
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

        let t = root;
        let point = ray.at(t);
        let normal = (point - current_center) / self.radius;
        let material = Arc::clone(&self.material);
        let (u, v) = self.get_sphere_uv(&normal);
        let mut record = HitRecord::new(point, normal, material, t, u, v, false);
        record.set_face_normal(ray, &normal);

        return Some(record);
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    use crate::color::Color;
    use crate::material::Lambertian;

    use super::*;

    #[test]
    fn test_create() {
        let center = Point3::new(0.0, 2.0, 1.0);
        let radius = 3.0;
        let material = Lambertian::from_albedo(Color::default());
        let sphere = Sphere::stationary(center, radius, Arc::new(material));
        assert_eq!(sphere.center.origin(), &center);
        assert_eq!(sphere.radius, radius);
    }

    #[test]
    fn test_create_with_negative_radius() {
        let center = Point3::new(0.0, 2.0, 1.0);
        let radius = -3.0;
        let material = Lambertian::from_albedo(Color::default());
        let sphere = Sphere::stationary(center, radius, Arc::new(material));
        assert_eq!(sphere.center.origin(), &center);
        assert_eq!(sphere.radius, 0.0);
    }

    #[test]
    fn test_ray_hits_sphere() {
        let sphere = Sphere::stationary(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::from_albedo(Color::default())),
        );

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 2.0);
        let interval = Interval::new(0.001, f64::INFINITY);
        let result = sphere.hit(&ray, &interval);
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
        let sphere = Sphere::stationary(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::from_albedo(Color::default())),
        );

        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 2.0);
        let interval = Interval::new(0.001, f64::INFINITY);
        let result = sphere.hit(&ray, &interval);
        assert_eq!(result.is_none(), true, "Ray should miss the sphere");
    }
}
