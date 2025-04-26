use rand::prelude::*;

use crate::vec3::{Point3, Vec3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [u8; POINT_COUNT],
    perm_y: [u8; POINT_COUNT],
    perm_z: [u8; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let randvec: [Vec3; POINT_COUNT] = std::array::from_fn(|_| Vec3::random_unit_vector());
        let perms: [[u8; POINT_COUNT]; 3] = std::array::from_fn(|_| {
            let mut array: [u8; POINT_COUNT] = std::array::from_fn(|i| i as u8);
            array.shuffle(&mut rng);
            array
        });
        Perlin {
            randvec,
            perm_x: perms[0],
            perm_y: perms[1],
            perm_z: perms[2],
        }
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as usize;
        let j = point.y.floor() as usize;
        let k = point.z.floor() as usize;

        let c: [[[Vec3; 2]; 2]; 2] = std::array::from_fn(|di| {
            std::array::from_fn(|dj| {
                std::array::from_fn(|dk| {
                    let idx = self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255];
                    self.randvec[idx as usize]
                })
            })
        });

        self.trilinear_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, point: &Point3, depth: u8) -> f64 {
        let mut accum = 0.0;
        let mut temp = point.clone();
        let mut weight = 0.5;

        for _ in 0..depth {
            accum += weight * self.noise(&temp);
            weight *= 0.5;
            temp = temp * 2.0;
        }

        accum.abs()
    }

    fn trilinear_interpolation(&self, c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        itertools::iproduct!(0..2, 0..2, 0..2)
            .map(|(i, j, k)| {
                (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                    * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                    * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                    * c[i][j][k].dot(&Vec3::new(u - i as f64, v - j as f64, w - k as f64))
            })
            .sum()
    }
}
