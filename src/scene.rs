use std::env;
use std::sync::Arc;

use clap::ValueEnum;
use rand::Rng;

use crate::bvh::BVHNode;
use crate::color::Color;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::quad::{Quad, box3d};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug, ValueEnum)]
pub enum Scene {
    BouncingSpheres,
    CheckeredSpheres,
    Earth,
    PerlinSpheres,
    Quads,
    SimpleLight,
    CornellBox,
    CornellSmoke,
    FinalScene,
}

impl Scene {
    pub fn build(&self) -> HittableList {
        match self {
            Scene::BouncingSpheres => bouncing_spheres(),
            Scene::CheckeredSpheres => checkered_spheres(),
            Scene::Earth => earth(),
            Scene::PerlinSpheres => perlin_spheres(),
            Scene::Quads => quads(),
            Scene::SimpleLight => simple_light(),
            Scene::CornellBox => cornell_box(),
            Scene::CornellSmoke => cornell_smoke(),
            Scene::FinalScene => final_scene(),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Scene::BouncingSpheres => "bouncing_spheres",
            Scene::CheckeredSpheres => "checkered_spheres",
            Scene::Earth => "earth",
            Scene::PerlinSpheres => "perlin_spheres",
            Scene::Quads => "quads",
            Scene::SimpleLight => "simple_light",
            Scene::CornellBox => "cornell_box",
            Scene::CornellSmoke => "cornell_smoke",
            Scene::FinalScene => "final_scene",
        }
    }
}

fn bouncing_spheres() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::from_albedo(Color::new(0.5, 0.5, 0.5)));
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
                    let material = Arc::new(Lambertian::from_albedo(albedo));
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
    let material2 = Arc::new(Lambertian::from_albedo(Color::new(0.4, 0.2, 0.1)));
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

fn checkered_spheres() -> HittableList {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));
    world
}

fn earth() -> HittableList {
    let mut world = HittableList::new();
    let earth_filename = env::var("EARTH_IMAGE_PATH").unwrap();
    let earth_texture = Arc::new(ImageTexture::new(earth_filename));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::stationary(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ));
    world.add(globe);
    world
}

fn perlin_spheres() -> HittableList {
    let mut world = HittableList::new();
    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perlin_texture.clone())),
    )));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));
    world
}

fn quads() -> HittableList {
    let mut world = HittableList::new();
    let left_red = Arc::new(Lambertian::from_albedo(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from_albedo(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from_albedo(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from_albedo(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from_albedo(Color::new(0.2, 0.8, 0.8)));
    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));
    world
}

fn simple_light() -> HittableList {
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perlin_texture.clone())),
    )));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));

    let diffuse_light = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diffuse_light,
    )));

    world
}

fn cornell_box() -> HittableList {
    let mut world = HittableList::new();
    let red = Arc::new(Lambertian::from_albedo(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_albedo(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_albedo(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = box3d(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let rotated_box1 = Arc::new(RotateY::new(box1, 15.0));
    let translated_box1 = Arc::new(Translate::new(rotated_box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(translated_box1);

    let box2 = box3d(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let rotated_box2 = Arc::new(RotateY::new(box2, -18.0));
    let translated_box2 = Arc::new(Translate::new(rotated_box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(translated_box2);

    world
}

fn cornell_smoke() -> HittableList {
    let mut world = HittableList::new();
    let red = Arc::new(Lambertian::from_albedo(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_albedo(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_albedo(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = box3d(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let rotated_box1 = Arc::new(RotateY::new(box1, 15.0));
    let translated_box1 = Arc::new(Translate::new(rotated_box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::from_albedo(
        translated_box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    let box2 = box3d(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let rotated_box2 = Arc::new(RotateY::new(box2, -18.0));
    let translated_box2 = Arc::new(Translate::new(rotated_box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::from_albedo(
        translated_box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    world
}

fn final_scene() -> HittableList {
    let mut world = HittableList::new();

    let mut boxes = HittableList::new();
    let ground = Arc::new(Lambertian::from_albedo(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::random_range(1.0..101.0);
            let z1 = z0 + w;
            boxes.add(box3d(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    world.add(Arc::new(BVHNode::new(&mut boxes)));

    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::from_albedo(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::stationary(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::stationary(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::from_albedo(
        boundary.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::stationary(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::from_albedo(
        boundary.clone(),
        1e-4,
        Color::new(1.0, 1.0, 1.0),
    )));

    let earth_filename = env::var("EARTH_IMAGE_PATH").unwrap();
    let earth_material = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(earth_filename))));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));
    let perlin_texture = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::stationary(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));

    let mut boxes = HittableList::new();
    let white = Arc::new(Lambertian::from_albedo(Color::new(0.73, 0.73, 0.73)));
    let number_of_spheres = 1000;
    for _ in 0..number_of_spheres {
        boxes.add(Arc::new(Sphere::stationary(
            Point3::random() * 165.0,
            10.0,
            white.clone(),
        )));
    }
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BVHNode::new(&mut boxes)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    world
}
