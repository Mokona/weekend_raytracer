use std::ops::{Add, AddAssign, MulAssign, Neg, SubAssign};

#[derive(Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new() -> Self {
        Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(t: (f64, f64, f64)) -> Self {
        Vector3 {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_debug_printed() {
        let v = Vector3::new();

        assert_eq!("Vector3 { x: 0.0, y: 0.0, z: 0.0 }", format!("{:?}", v));
    }

    #[test]
    fn can_be_added() {
        let v1 = Vector3::from((0., 0., 0.));
        let v2 = Vector3::from((1., 2., 3.));
        let v3 = Vector3::from((1., 2., 3.));

        assert_eq!(v3, v1 + v2);
    }

    #[test]
    fn can_be_negated() {
        let v1 = Vector3::from((1., 2., 3.));
        let v2 = Vector3::from((-1., -2., -3.));

        assert_eq!(v2, -v1);
    }

    #[test]
    fn can_be_mutably_added() {
        let mut v1 = Vector3::from((0., 0., 0.));
        let v2 = Vector3::from((1., 2., 3.));
        let v3 = Vector3::from((1., 2., 3.));

        v1 += v2;

        assert_eq!(v3, v1);
    }

    #[test]
    fn can_be_mutably_substracted() {
        let mut v1 = Vector3::from((0., 0., 0.));
        let v2 = Vector3::from((1., 2., 3.));
        let v3 = Vector3::from((-1., -2., -3.));

        v1 -= v2;

        assert_eq!(v3, v1);
    }

    #[test]
    fn can_be_mutably_multiplied_by_scalar() {
        let mut v1 = Vector3::from((1., 2., 3.));
        let v2 = Vector3::from((2., 4., 6.));

        v1 *= 2.;

        assert_eq!(v2, v1);
    }
}
