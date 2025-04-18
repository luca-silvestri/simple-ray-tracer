mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use core::f64;
use std::{io, sync::Arc};

use interval::Interval;
use log::info;

use color::{Color, write_color};
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut record = HitRecord::default();
    if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut record) {
        return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    env_logger::init();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Image
    //
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let mut image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World
    //
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    //
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / image_width.into();
    let pixel_delta_v = viewport_v / image_height.into();
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    //
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray, &world);
            write_color(&mut out, &pixel_color).unwrap();
        }
    }
    info!("Done.");
}
