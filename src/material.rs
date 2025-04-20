use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }
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
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = Vec3::reflect(ray_in.direction(), &record.normal);
        reflected = reflected.unit_vector() + self.fuzz * Vec3::random_unit_vector();
        let attenuation = self.albedo;
        let ray_out = Ray::new(record.point, reflected);
        return Some((attenuation, ray_out));
    }
}
