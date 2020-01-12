use crate::ray::Ray;
use crate::vector3::Vector3;

pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            lower_left_corner: Vector3::from((-2., -1., -1.)),
            horizontal: Vector3::from((4., 0., 0.)),
            vertical: Vector3::from((0., 2., 0.)),
            origin: Vector3::new(),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v,
        )
    }
}
