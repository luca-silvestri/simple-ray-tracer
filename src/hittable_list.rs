use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: AABB {
                x: Interval::empty(),
                y: Interval::empty(),
                z: Interval::empty(),
            },
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = self.bbox.union(object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if let Some(record) = object.hit(ray, &Interval::new(interval.min, closest_so_far)) {
                closest_so_far = record.t;
                result = Some(record);
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        material::Material,
        ray::Ray,
        sphere::Sphere,
        vec3::{Point3, Vec3},
    };

    struct TestMaterial;
    impl Material for TestMaterial {}

    #[test]
    fn test_ray_hits_one_sphere_in_list() {
        let mut world = HittableList::new();
        let sphere = Arc::new(Sphere::stationary(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(TestMaterial),
        ));
        world.add(sphere);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 2.0);
        let result = world.hit(&ray, &Interval::new(0.001, f64::INFINITY));
        assert_eq!(
            result.is_some(),
            true,
            "Ray should hit the sphere in the world"
        );
        let record = result.unwrap();
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
        let sphere = Arc::new(Sphere::stationary(
            Vec3::new(0.0, 0.0, -5.0),
            0.5,
            Arc::new(TestMaterial),
        ));
        world.add(sphere);
        let ray = Ray::new(Point3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 2.0);
        let result = world.hit(&ray, &Interval::new(0.001, f64::INFINITY));
        assert_eq!(
            result.is_none(),
            true,
            "Ray should miss all objects in the world"
        );
    }
}
