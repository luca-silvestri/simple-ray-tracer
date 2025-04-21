use std::io::Write;

use rand::Rng;

use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vertical_field_of_view: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub view_up: Vec3,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vertical_field_of_view: f64,
        lookfrom: Point3,
        lookat: Point3,
        view_up: Vec3,
    ) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vertical_field_of_view,
            lookfrom,
            lookat,
            view_up,
            image_height: 0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
        };
        camera.initialize();
        camera
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.lookfrom;
        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = self.vertical_field_of_view.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.view_up.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (focal_length * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render<W: Write>(&self, world: &impl Hittable, out: &mut W) {
        write!(out, "P3\n{} {}\n255\n", self.image_width, self.image_height).unwrap();
        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&ray, self.max_depth, world);
                }
                pixel_color = pixel_color / self.samples_per_pixel as f64;
                write_color(out, &pixel_color).unwrap();
            }
        }
        eprintln!("Done.")
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let mut rng = rand::rng();
        let offset = Vec3::new(
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5),
            0.0,
        );
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        Ray::new(self.center, pixel_sample - self.center)
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        match world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            Some(record) => match record.material.scatter(ray, &record) {
                Some((attenuation, scattered_ray)) => {
                    attenuation * self.ray_color(&scattered_ray, depth - 1, world)
                }
                None => Color::new(0.0, 0.0, 0.0),
            },
            None => {
                let unit_direction = ray.direction().unit_vector();
                let a = 0.5 * (unit_direction.y + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}
