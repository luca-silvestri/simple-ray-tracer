use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let bbox = Quad::set_bounding_box(q, u, v);
        let n = u.cross(&v);
        let normal = n.unit_vector();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);
        Quad {
            q,
            u,
            v,
            w,
            material,
            bbox,
            normal,
            d,
        }
    }

    fn set_bounding_box(q: Point3, u: Vec3, v: Vec3) -> AABB {
        let bbox1 = AABB::from_extremes(&q, &(q + u + v));
        let bbox2 = AABB::from_extremes(&(q + u), &(q + v));
        bbox1.union(&bbox2)
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot(&ray.direction());
        if denominator.abs() < 1e-8 {
            return None;
        }
        let t = (self.d - self.normal.dot(&ray.origin())) / denominator;
        if !interval.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let p = intersection - self.q;
        let alpha = self.w.dot(&p.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&p));
        if !(0.0..=1.0).contains(&alpha) || !(0.0..=1.0).contains(&beta) {
            return None;
        }

        let mut record = HitRecord::new(
            intersection,
            self.normal,
            Arc::clone(&self.material),
            t,
            alpha,
            beta,
            false,
        );
        record.set_face_normal(ray, &self.normal);
        Some(record)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
