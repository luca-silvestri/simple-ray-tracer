use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(world: &mut HittableList) -> BVHNode {
        let size = world.objects.len();
        BVHNode::from_hittable_list(&mut world.objects, 0, size)
    }

    fn from_hittable_list(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
    ) -> BVHNode {
        let mut bbox = AABB::empty();
        bbox = (start..end).fold(bbox, |bbox, index| {
            bbox.union(objects[index].bounding_box())
        });
        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => BVHNode::box_compare_x,
            1 => BVHNode::box_compare_y,
            _ => BVHNode::box_compare_z,
        };
        let span = end - start;
        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>);
        match span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + span / 2;
                left = Arc::new(BVHNode::from_hittable_list(objects, start, mid));
                right = Arc::new(BVHNode::from_hittable_list(objects, mid, end));
            }
        }

        BVHNode { left, right, bbox }
    }

    fn box_compare_x(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        a.bounding_box()
            .x
            .min
            .partial_cmp(&b.bounding_box().x.min)
            .unwrap()
    }
    fn box_compare_y(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        a.bounding_box()
            .y
            .min
            .partial_cmp(&b.bounding_box().y.min)
            .unwrap()
    }
    fn box_compare_z(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        a.bounding_box()
            .z
            .min
            .partial_cmp(&b.bounding_box().z.min)
            .unwrap()
    }
}

impl Hittable for BVHNode {
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, interval) {
            return None;
        }

        let hit_left = self.left.hit(ray, interval);
        let closest_so_far = match &hit_left {
            Some(record) => record.t,
            None => interval.max,
        };
        let hit_right = self
            .right
            .hit(ray, &Interval::new(interval.min, closest_so_far));

        hit_right.or(hit_left)
    }
}
