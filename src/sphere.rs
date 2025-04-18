use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius: f64::max(radius, 0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - *ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let root = (h - sqrtd) / a;
        if root <= tmin || root >= tmax {
            let root = (h + sqrtd) / a;
            if root <= tmin || root >= tmax {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        let normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &normal);

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let center = Point3::new(0.0, 2.0, 1.0);
        let radius = 3.0;
        let sphere = Sphere::new(center, radius);
        assert_eq!(sphere.center, center);
        assert_eq!(sphere.radius, radius);
    }

    #[test]
    fn test_create_with_negative_radius() {
        let center = Point3::new(0.0, 2.0, 1.0);
        let radius = -3.0;
        let sphere = Sphere::new(center, radius);
        assert_eq!(sphere.center, center);
        assert_eq!(sphere.radius, 0.0);
    }

    #[test]
    fn test_ray_hits_sphere() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let tmin = 0.001;
        let tmax = f64::INFINITY;
        let mut record = HitRecord::default();

        let hit = sphere.hit(&ray, tmin, tmax, &mut record);

        assert!(hit, "Ray should hit the sphere");
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
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        let tmin = 0.001;
        let tmax = f64::INFINITY;
        let mut record = HitRecord::default();

        let hit = sphere.hit(&ray, tmin, tmax, &mut record);

        assert!(!hit, "Ray should miss the sphere");
    }
}
