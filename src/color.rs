use std::fmt;
use std::fmt::Formatter;

pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
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
}
