use rand::random;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, _ray_in: &Ray, _record: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal
        }
        let attenuation = self.albedo;
        let ray_out = Ray::new(record.point, scatter_direction);
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
        let scattered = Ray::new(record.point, reflected);
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
        let scattered = Ray::new(record.point, direction);
        Some((attenuation, scattered))
    }
}
