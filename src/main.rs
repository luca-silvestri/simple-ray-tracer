use std::{io, sync::Arc};

use rand::Rng;

use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::hittable_list::HittableList;
use ray_tracer::material::{Dielectric, Lambertian, Metal};
use ray_tracer::sphere::Sphere;
use ray_tracer::vec3::{Point3, Vec3};

fn main() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rng.random_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.random_range(0.0..1.0),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                } else if choose_material < 0.95 {
                    let albedo = Color::random();
                    let fuzz = rng.random_range(0.0..0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let samples_per_pixel: i32 = 10;
    let max_depth = 50;
    let vertical_field_of_view = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_distance = 10.0;
    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vertical_field_of_view,
        lookfrom,
        lookat,
        view_up,
        defocus_angle,
        focus_distance,
    );
    camera.render(&world, &mut io::stdout());
}
