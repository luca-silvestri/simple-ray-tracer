mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use std::{io, sync::Arc};

use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let samples_per_pixel: i32 = 100;
    let max_depth = 50;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    camera.render(&world, &mut io::stdout());
}
