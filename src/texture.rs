use std::sync::Arc;

use crate::{color::Color, vec3::Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        SolidColor { albedo }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        SolidColor::new(Color::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: &Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, color1: Color, color2: Color) -> Self {
        CheckerTexture::new(
            scale,
            Arc::new(SolidColor::new(color1)),
            Arc::new(SolidColor::new(color2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color {
        let xint = f64::floor(self.inv_scale * point.x) as i32;
        let yint = f64::floor(self.inv_scale * point.y) as i32;
        let zint = f64::floor(self.inv_scale * point.z) as i32;

        let is_even = (xint + yint + zint) % 2 == 0;
        match is_even {
            true => self.even.value(u, v, point),
            false => self.odd.value(u, v, point),
        }
    }
}
