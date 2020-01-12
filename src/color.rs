use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn lerp(self, other: Color, t: f32) -> Color {
        assert!(t <= 1.);
        assert!(t >= 0.);

        let anti_t = 1. - t;

        Color {
            r: (self.r as f32 * anti_t + other.r as f32 * t) as u8,
            g: (self.g as f32 * anti_t + other.g as f32 * t) as u8,
            b: (self.b as f32 * anti_t + other.b as f32 * t) as u8,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Color::from((r as f64, g as f64, b as f64))
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        let ir = (255.99 * r) as u8;
        let ig = (255.99 * g) as u8;
        let ib = (255.99 * b) as u8;

        Color::new(ir, ig, ib)
    }
}

impl<T> Into<(T, T, T)> for Color
where
    T: From<u8>,
{
    fn into(self) -> (T, T, T) {
        (self.r.into(), self.g.into(), self.b.into())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn can_be_formatted() {
        let c = Color::new(10, 20, 255);

        assert_eq!("10 20 255", format!("{}", c));
    }

    #[test]
    fn can_be_created_from_f32() {
        let c1 = Color::from((1.0f32, 1.0f32, 1.0f32));
        let c2 = Color::new(255, 255, 255);

        assert_eq!(c2, c1);
    }

    #[test]
    fn can_be_created_from_f54() {
        let c1 = Color::from((1.0f64, 1.0f64, 1.0f64));
        let c2 = Color::new(255, 255, 255);

        assert_eq!(c2, c1);
    }

    #[test]
    fn can_lerp_with_another_color() {
        let c1 = Color::new(0, 0, 0);
        let c2 = Color::new(255, 255, 255);
        let c3 = Color::new(127, 127, 127);

        assert_eq!(c2, c1.lerp(c2, 1.));
        assert_eq!(c1, c1.lerp(c2, 0.));
        assert_eq!(c3, c1.lerp(c2, 0.5));
    }

    #[test]
    fn can_be_transformed_into_a_tuple() {
        let c2 = Color::new(255, 100, 50);
        let t: (u32, u32, u32) = c2.into();

        assert_eq!((255, 100, 50), t);
    }

    #[test]
    fn can_be_transformed_into_a_f64_tuple() {
        let c2 = Color::new(255, 100, 50);
        let t: (f64, f64, f64) = c2.into();

        assert_eq!((255., 100., 50.), t);
    }
}
