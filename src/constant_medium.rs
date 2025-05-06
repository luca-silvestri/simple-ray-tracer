use core::f64;
use std::sync::Arc;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::{Isotropic, Material},
    ray::Ray,
    texture::Texture,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    negative_inverse_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> Self {
        let negative_inverse_density = -1.0 / density;
        let phase_function = Arc::new(Isotropic::new(texture));
        ConstantMedium {
            boundary,
            negative_inverse_density,
            phase_function,
        }
    }

    pub fn from_albedo(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        let negative_inverse_density = -1.0 / density;
        let phase_function = Arc::new(Isotropic::from_albedo(albedo));
        ConstantMedium {
            boundary,
            negative_inverse_density,
            phase_function,
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self) -> &crate::aabb::AABB {
        &self.boundary.bounding_box()
    }

    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut record1 = self.boundary.hit(ray, &Interval::universe())?;
        let mut record2 = self
            .boundary
            .hit(ray, &Interval::new(record1.t + 1e-4, f64::INFINITY))?;
        record1.t = f64::max(record1.t, interval.min);
        record2.t = f64::min(record2.t, interval.max);
        record1.t = f64::max(record1.t, 0.0);
        let ray_length = ray.direction().length();
        let distance_inside_boundary = (record2.t - record1.t) * ray_length;
        let hit_distance = self.negative_inverse_density * rand::random::<f64>().ln();
        if hit_distance > distance_inside_boundary {
            return None;
        }
        let t = record1.t + hit_distance / ray_length;
        let point = ray.at(t);
        let normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        let front_face = true; // arbitrary
        let material = self.phase_function.clone();
        Some(HitRecord::new(
            point, normal, material, t, 0.0, 0.0, front_face,
        ))
    }
}
