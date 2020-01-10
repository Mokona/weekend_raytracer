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

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn norm(&self) -> f64 {
        self.squared_norm().sqrt()
    }
    pub fn squared_norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalize(&mut self) {
        let n = self.norm();
        assert_ne!(n, 0.);

        *self *= 1. / n;
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
    fn can_be_mutably_subtracted() {
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

    #[test]
    fn can_give_dot_product() {
        let v1 = Vector3::from((1., 0., 0.));
        let v2 = Vector3::from((0., 1., 0.));
        let v3 = Vector3::from((0., 0., 1.));

        assert_eq!(0., v1.dot(&v2));
        assert_eq!(0., v1.dot(&v3));
        assert_eq!(0., v2.dot(&v3));
    }

    #[test]
    fn can_give_cross_product() {
        let v1 = Vector3::from((1., 0., 0.));
        let v2 = Vector3::from((0., 1., 0.));
        let v3 = Vector3::from((0., 0., 1.));

        assert_eq!(v3, v1.cross(&v2));
        assert_eq!(-v3, v2.cross(&v1));
    }

    #[test]
    fn can_give_norm() {
        let v1 = Vector3::from((1., 0., 0.));
        let v2 = Vector3::from((2., 1., 3.));

        assert_eq!(1., v1.norm());
        assert_eq!(14., v2.squared_norm());
    }

    #[test]
    fn can_normalize_self() {
        let mut v1 = Vector3::from((2., 0., 0.));
        let v2 = Vector3::from((1., 0., 0.));

        v1.normalize();

        assert_eq!(v2, v1);
    }
}
