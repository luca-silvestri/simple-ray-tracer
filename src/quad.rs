use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
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

pub fn box3d(a: Point3, b: Point3, material: Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::new();
    let min = Point3::new(f64::min(a.x, b.x), f64::min(a.y, b.y), f64::min(a.z, b.z));
    let max = Point3::new(f64::max(a.x, b.x), f64::max(a.y, b.y), f64::max(a.z, b.z));
    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        material.clone(),
    )));
    Arc::new(sides)
}
