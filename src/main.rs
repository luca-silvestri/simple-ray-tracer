mod color;
mod ray;
mod vec3;

use std::io;

use log::info;

use color::{Color, write_color};
use ray::Ray;
use vec3::{Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc: Vec3 = *center - *ray.origin();
    let a = ray.direction().length_squared();
    let h = ray.direction().dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    };
}

fn ray_color(ray: &Ray) -> Color {
    let center = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&center, 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t) - center).unit_vector();
        return 0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
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
            let pixel_color = ray_color(&ray);
            write_color(&mut out, &pixel_color).unwrap();
        }
    }
    info!("Done.");
}
