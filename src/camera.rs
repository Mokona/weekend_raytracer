use crate::ray::Ray;
use crate::vector3::Vector3;
use std::f64;

pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
}

type Degrees = f64;

impl Camera {
    pub fn new(vertical_fov: Degrees, aspect_ratio: f64) -> Self {
        let theta = vertical_fov * f64::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect_ratio * half_height;

        Camera {
            lower_left_corner: Vector3::from((-half_width, -half_height, -1.)),
            horizontal: Vector3::from((2. * half_width, 0., 0.)),
            vertical: Vector3::from((0., 2. * half_height, 0.)),
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
