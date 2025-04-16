mod color;
mod ray;
mod vec3;

use std::io;

use log::info;

use color::{Color, write_color};

fn main() {
    env_logger::init();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Image
    //
    let image_width: i16 = 256;
    let image_height: i16 = 256;

    // Render
    //
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel = Color::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.0,
            );

            write_color(&mut out, &pixel).unwrap();
        }
    }
    info!("Done.");
}
