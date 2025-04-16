fn main() {
    // Image
    //
    let image_width: i16 = 256;
    let image_height: i16 = 256;

    // Render
    //
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        for i in 0..image_width {
            let r: f64 = (i as f64) / ((image_width - 1) as f64);
            let g: f64 = (j as f64) / ((image_height - 1) as f64);
            let b: f64 = 0.0;

            let ir: i16 = (255.999 * r) as i16;
            let ig: i16 = (255.999 * g) as i16;
            let ib: i16 = (255.999 * b) as i16;

            print!("{ir} {ig} {ib}\n")
        }
    }
}
