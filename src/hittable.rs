use core::f64;
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

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box().clone() + offset;
        Translate {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(*ray.origin() - self.offset, *ray.direction(), ray.time());
        match self.object.hit(&offset_ray, interval) {
            Some(mut record) => {
                record.point = record.point + self.offset;
                Some(record)
            }
            None => None,
        }
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, theta: f64) -> Self {
        let radians = theta.to_radians();
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);

        let bbox = object.bounding_box().clone();
        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        (min, max) = itertools::iproduct!(0..2, 0..2, 0..2)
            .map(|(i, j, k)| {
                let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;
                let new_x = cos_theta * x + sin_theta * z;
                let new_z = -sin_theta * x + cos_theta * z;
                Point3::new(new_x, y, new_z)
            })
            .fold((min, max), |(min, max), p| (min.min(&p), max.max(&p)));
        let bbox = AABB::from_extremes(&min, &max);

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let origin = Point3::new(
            (self.cos_theta * ray.origin().x) - (self.sin_theta * ray.origin().z),
            ray.origin().y,
            (self.sin_theta * ray.origin().x) + (self.cos_theta * ray.origin().z),
        );
        let direction = Point3::new(
            (self.cos_theta * ray.direction().x) - (self.sin_theta * ray.direction().z),
            ray.direction().y,
            (self.sin_theta * ray.direction().x) + (self.cos_theta * ray.direction().z),
        );
        let rotated_ray = Ray::new(origin, direction, ray.time());
        match self.object.hit(&rotated_ray, interval) {
            Some(mut record) => {
                record.point = Point3::new(
                    (self.cos_theta * record.point.x) + (self.sin_theta * record.point.z),
                    record.point.y,
                    (-self.sin_theta * record.point.x) + (self.cos_theta * record.point.z),
                );
                record.normal = Vec3::new(
                    (self.cos_theta * record.normal.x) + (self.sin_theta * record.normal.z),
                    record.normal.y,
                    (-self.sin_theta * record.normal.x) + (self.cos_theta * record.normal.z),
                );
                Some(record)
            }
            None => None,
        }
    }
}
