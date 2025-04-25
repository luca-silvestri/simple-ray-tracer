use std::{
    env,
    fs::File,
    io::{self, BufReader},
    sync::Arc,
};

use dotenv::dotenv;
use rand::Rng;

use ray_tracer::color::Color;
use ray_tracer::hittable_list::HittableList;
use ray_tracer::material::{Dielectric, Lambertian, Metal};
use ray_tracer::sphere::Sphere;
use ray_tracer::vec3::{Point3, Vec3};
use ray_tracer::{
    bvh::BVHNode,
    camera::{Camera, CameraSettings},
};

fn main() {
    dotenv().ok();
    let scene = build_scene();
    let camera = build_camera();
    camera.render(&scene, &mut io::stdout());
}

fn build_camera() -> Camera {
    let settings_path = env::var("CAMERA_SETTINGS_PATH").unwrap();
    let file = File::open(settings_path).unwrap();
    let reader = BufReader::new(file);
    let settings: CameraSettings = serde_json::from_reader(reader).unwrap();
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
    )
}

fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::stationary(
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
                    let end_center = center + Vec3::new(0.0, rng.random_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::moving(center, end_center, 0.2, material)));
                } else if choose_material < 0.95 {
                    let albedo = Color::random();
                    let fuzz = rng.random_range(0.0..0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::stationary(center, 0.2, material)));
                } else {
                    let material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::stationary(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut scene = HittableList::new();
    scene.add(Arc::new(BVHNode::new(&mut world)));
    scene
}
