use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        AABB { x, y, z }
    }

    pub fn union(&self, other: &AABB) -> Self {
        AABB {
            x: Interval::from_intervals(&self.x, &other.x),
            y: Interval::from_intervals(&self.y, &other.y),
            z: Interval::from_intervals(&self.z, &other.z),
        }
    }

    pub fn from_extremes(a: &Point3, b: &Point3) -> Self {
        let make_interval = |a: f64, b: f64| Interval::new(a.min(b), a.max(b));
        AABB {
            x: make_interval(a.x, b.x),
            y: make_interval(a.y, b.y),
            z: make_interval(a.z, b.z),
        }
    }

    pub fn hit(&self, ray: &Ray, t: &Interval) -> bool {
        let Point3 {
            x: px,
            y: py,
            z: pz,
        } = ray.origin();
        let Vec3 {
            x: dx,
            y: dy,
            z: dz,
        } = ray.direction();
        for (interval, direction, origin) in
            [(&self.x, dx, px), (&self.y, dy, py), (&self.z, dz, pz)]
        {
            let t0 = (interval.min - origin) / direction;
            let t1 = (interval.max - origin) / direction;
            let t_min = f64::max(t.min, f64::min(t0, t1));
            let t_max = f64::min(t.max, f64::max(t0, t1));
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
