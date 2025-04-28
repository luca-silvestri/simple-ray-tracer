use std::env;
use std::sync::Arc;

use clap::ValueEnum;
use rand::Rng;

use crate::bvh::BVHNode;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::quad::Quad;
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
