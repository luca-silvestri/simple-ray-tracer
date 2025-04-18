use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool {
        let mut temp: HitRecord = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = tmax;

        for object in &self.objects {
            if object.hit(ray, tmin, closest_so_far, &mut temp) {
                hit_anything = true;
                closest_so_far = temp.t;
                *record = temp;
            }
        }

        hit_anything
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Vec3, ray::Ray, sphere::Sphere, vec3::Point3};

    #[test]
    fn test_ray_hits_one_sphere_in_list() {
        let mut world = HittableList::new();
        let sphere = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
        world.add(sphere);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let tmin = 0.001;
        let tmax = f64::INFINITY;
        let mut record = HitRecord::default();
        let hit = world.hit(&ray, tmin, tmax, &mut record);
        assert!(hit, "Ray should hit the sphere in the world");
        assert!(
            (record.t - 0.5).abs() < 1e-6,
            "Expected hit at t ~ 0.5, got {}",
            record.t
        );
        assert!(
            (record.point - Vec3::new(0.0, 0.0, -0.5)).length() < 1e-6,
            "Unexpected hit point: {:?}",
            record.point
        );
    }

    #[test]
    fn test_ray_misses_all_objects() {
        let mut world = HittableList::new();
        let sphere = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -5.0), 0.5));
        world.add(sphere);
        let ray = Ray::new(Point3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut record = HitRecord::default();
        let hit = world.hit(&ray, 0.001, f64::INFINITY, &mut record);
        assert!(!hit, "Ray should miss all objects in the world");
    }
}
