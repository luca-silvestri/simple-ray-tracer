use std::io::{self, Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel: &Color) -> io::Result<()> {
    let r = pixel.x;
    let g = pixel.y;
    let b = pixel.z;

    let rbyte = (255.999 * r) as i16;
    let gbyte = (255.999 * g) as i16;
    let bbyte = (255.999 * b) as i16;

    write!(out, "{rbyte} {gbyte} {bbyte}\n")?;

    Ok(())
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
