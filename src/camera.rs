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
    pub fn new(
        look_from: Vector3,
        look_at: Vector3,
        up_vector: Vector3,
        vertical_fov: Degrees,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vertical_fov * f64::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).normalized();
        let u = up_vector.cross(&w);
        let v = w.cross(&u);

        let origin = look_from;

        Camera {
            lower_left_corner: origin - (u * half_width) - (v * half_height) - w,
            horizontal: u * (2. * half_width),
            vertical: v * (2. * half_height),
            origin,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            (self.lower_left_corner + self.horizontal * u + self.vertical * v) - self.origin,
        )
    }
}
