use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn format_color(pixel: &Color) -> String {
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

    format!("{rbyte} {gbyte} {bbyte}")
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    f64::max(linear_component, 0.0).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_color() {
        let color = Color::new(0.5, 0.7, 0.9);
        format_color(&color);
        let expected = "127 179 230\n";
        assert_eq!(format_color(&color), expected)
    }
}
