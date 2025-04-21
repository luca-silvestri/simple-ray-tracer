use std::io::{self, Write};

use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel: &Color) -> io::Result<()> {
    let mut r = pixel.x;
    let mut g = pixel.y;
    let mut b = pixel.z;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (255.999 * intensity.clamp(r)) as i16;
    let gbyte = (255.999 * intensity.clamp(g)) as i16;
    let bbyte = (255.999 * intensity.clamp(b)) as i16;

    write!(out, "{rbyte} {gbyte} {bbyte}\n")?;

    Ok(())
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write_color() {
        let color = Color::new(0.5, 0.7, 0.9);
        let mut output = Cursor::new(Vec::new());

        write_color(&mut output, &color).unwrap();

        let expected = "127 179 230\n";
        let actual = String::from_utf8(output.into_inner()).unwrap();

        assert_eq!(actual, expected)
    }
}
