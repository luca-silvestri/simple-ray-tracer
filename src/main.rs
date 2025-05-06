use std::{env, fs::File, io::BufReader, path::Path};

use clap::Parser;
use dotenv::dotenv;

use ray_tracer::camera::{Camera, CameraSettings};
use ray_tracer::scene::Scene;

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_enum)]
    scene: Scene,

    #[arg(short, long, default_value = "image.ppm")]
    output: String,
}

fn build_camera(scene: Scene) -> Camera {
    let settings_dir = env::var("CAMERA_SETTINGS_DIRECTORY").unwrap();
    let settings_path = Path::new(&settings_dir).join(format!("{}.json", scene.to_str()));
    let file = File::open(settings_path).unwrap();
    let reader = BufReader::new(file);
    let settings: CameraSettings = serde_json::from_reader(reader).unwrap();
    Camera::from_settings(settings)
}

fn main() {
    dotenv().ok();
    let args = Args::parse();
    let scene = args.scene.build();
    let camera = build_camera(args.scene);
    let mut output = File::create(args.output).unwrap();
    camera.render(&scene, &mut output);
}
