use rand::prelude::*;

use crate::vec3::{Point3, Vec3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    random_vectors: [Vec3; POINT_COUNT],
    permutation_x: [usize; POINT_COUNT],
    permutation_y: [usize; POINT_COUNT],
    permutation_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let random_vectors: [Vec3; POINT_COUNT] =
            std::array::from_fn(|_| Vec3::random_unit_vector());
        let permutations: [[usize; POINT_COUNT]; 3] = std::array::from_fn(|_| {
            let mut array: [usize; POINT_COUNT] = std::array::from_fn(|i| i);
            array.shuffle(&mut rng);
            array
        });
        Perlin {
            random_vectors,
            permutation_x: permutations[0],
            permutation_y: permutations[1],
            permutation_z: permutations[2],
        }
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let c: [[[Vec3; 2]; 2]; 2] = std::array::from_fn(|di| {
            std::array::from_fn(|dj| {
                std::array::from_fn(|dk| {
                    let idx = self.permutation_x[((i + di as i32) & 255) as usize]
                        ^ self.permutation_y[((j + dj as i32) & 255) as usize]
                        ^ self.permutation_z[((k + dk as i32) & 255) as usize];
                    self.random_vectors[idx]
                })
            })
        });

        self.perlin_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, point: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut loc = point.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&loc);
            weight = 0.5 * weight;
            loc = 2.0 * loc;
        }

        f64::abs(accum)
    }

    fn perlin_interpolation(&self, c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        itertools::iproduct!(0..2, 0..2, 0..2)
            .map(|(i, j, k)| {
                let weight_vector = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                let weight_scalar = (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                    * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                    * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww));
                weight_scalar * c[i][j][k].dot(&weight_vector)
            })
            .sum()
    }
}
