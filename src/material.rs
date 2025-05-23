use std::sync::Arc;

use rand::random;

use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::{Point3, Vec3},
};

pub trait Material: Send + Sync {
    fn scatter(&self, _ray_in: &Ray, _record: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn emitted(&self, _u: f64, _v: f64, _point: &Point3) -> Color {
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Lambertian { texture }
    }

    pub fn from_albedo(albedo: Color) -> Self {
        Lambertian::new(Arc::new(SolidColor::new(albedo)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal
        }
        let attenuation = self.texture.value(record.u, record.v, &record.point);
        let ray_out = Ray::new(record.point, scatter_direction, ray_in.time());
        return Some((attenuation, ray_out));
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: f64::min(fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = Vec3::reflect(ray_in.direction(), &record.normal);
        reflected = reflected.unit_vector() + self.fuzz * Vec3::random_unit_vector();
        let attenuation = self.albedo;
        let scattered = Ray::new(record.point, reflected, ray_in.time());
        if scattered.direction().dot(&record.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_index = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = -unit_direction.dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_index * sin_theta > 1.0;
        let reflectance_threshold: f64 = random();
        let does_reflect = self.reflectance(cos_theta, refraction_index) > reflectance_threshold;
        let direction = if cannot_refract || does_reflect {
            Vec3::reflect(&unit_direction, &record.normal)
        } else {
            Vec3::refract(&unit_direction, &record.normal, refraction_index)
        };
        let scattered = Ray::new(record.point, direction, ray_in.time());
        Some((attenuation, scattered))
    }
}

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        DiffuseLight { texture }
    }

    pub fn from_color(color: Color) -> Self {
        let texture = Arc::new(SolidColor::new(color));
        DiffuseLight { texture }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, point: &Point3) -> Color {
        self.texture.value(u, v, point)
    }
}

pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Isotropic { texture }
    }

    pub fn from_albedo(albedo: Color) -> Self {
        Isotropic::new(Arc::new(SolidColor::new(albedo)))
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(record.point, Vec3::random_unit_vector(), ray_in.time());
        let attenuation = self.texture.value(record.u, record.v, &record.point);
        Some((attenuation, scattered))
    }
}
