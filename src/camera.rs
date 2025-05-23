use std::cmp;
use std::io::Write;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    color::{Color, format_color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Serialize, Deserialize)]
pub struct CameraSettings {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vertical_field_of_view: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub view_up: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
    pub background: Color,
}

pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    defocus_angle: f64,
    background: Color,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
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
        defocus_angle: f64,
        focus_distance: f64,
        background: Color,
    ) -> Self {
        let image_height = cmp::max((image_width as f64 / aspect_ratio) as i32, 1);

        let center = lookfrom;

        let theta = vertical_field_of_view.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (lookfrom - lookat).unit_vector();
        let u = view_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - (focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            background,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn from_settings(settings: CameraSettings) -> Self {
        Camera::new(
            settings.aspect_ratio,
            settings.image_width,
            settings.samples_per_pixel,
            settings.max_depth,
            settings.vertical_field_of_view,
            settings.lookfrom,
            settings.lookat,
            settings.view_up,
            settings.defocus_angle,
            settings.focus_distance,
            settings.background,
        )
    }

    pub fn render<W: Write>(&self, world: &(impl Hittable + Send + Sync), out: &mut W) {
        let progress_bar = self.get_progress_bar();
        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(i32, i32)>>()
            .into_par_iter()
            .progress_with(progress_bar)
            .map(|(j, i)| {
                let pixel_color: Color = (0..self.samples_per_pixel)
                    .into_iter()
                    .map(|_| self.ray_color(&self.get_ray(i, j), self.max_depth, world))
                    .sum::<Color>()
                    / self.samples_per_pixel as f64;
                format_color(&pixel_color)
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            out,
            "P3\n{} {}\n255\n{}",
            self.image_width, self.image_height, pixels
        )
        .unwrap();
    }

    fn get_progress_bar(&self) -> ProgressBar {
        let total_number_of_pixels = self.image_width as u64 * self.image_height as u64;
        let progress_bar = ProgressBar::new(total_number_of_pixels);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        progress_bar
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
        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;
        let time = rng.random_range(0.0..1.0);
        Ray::new(origin, direction, time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let point = Point3::random_in_unit_disk();
        self.center + (point.x * self.defocus_disk_u) + (point.y * self.defocus_disk_v)
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        match world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            Some(record) => {
                let color_from_emission =
                    record.material.emitted(record.u, record.v, &record.point);
                match record.material.scatter(ray, &record) {
                    Some((attenuation, scattered_ray)) => {
                        let color_from_scatter =
                            attenuation * self.ray_color(&scattered_ray, depth - 1, world);
                        color_from_emission + color_from_scatter
                    }
                    None => color_from_emission,
                }
            }
            None => self.background,
        }
    }
}
