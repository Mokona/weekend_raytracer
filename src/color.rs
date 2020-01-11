use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        let ir = (255.99 * r) as u8;
        let ig = (255.99 * g) as u8;
        let ib = (255.99 * b) as u8;

        Color::new(ir, ig, ib)
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
    fn can_be_created_from_doubles() {
        let c1 = Color::from((1., 1., 1.));
        let c2 = Color::new(255, 255, 255);

        assert_eq!(c2, c1);
    }
}
