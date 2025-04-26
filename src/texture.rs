use std::sync::Arc;

use image::{DynamicImage, GenericImageView, ImageReader};

use crate::{color::Color, interval::Interval, perlin::Perlin, vec3::Point3};

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

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: String) -> Self {
        let image = ImageReader::open(filename).unwrap().decode().unwrap();
        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: &Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 0.0);
        }
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);
        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        let pixel = self.image.get_pixel(i, j);
        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * *point)))
        // Color::new(0.5, 0.5, 0.5)
        //     * (1.0 + f64::sin(self.scale * point.z + 10.0 * self.noise.turbulence(point, 7)))
    }
}
