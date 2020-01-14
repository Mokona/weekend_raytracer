use crate::ray::Ray;
use crate::vector3::{random_in_unit_sphere, Vector3};
use std::f64;

pub struct Basis {
    u: Vector3,
    v: Vector3,
    w: Vector3,
}

pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
    orthonormal_basis: Basis,
    lens_radius: f64,
}

type Degrees = f64;

impl Camera {
    pub fn new(
        look_from: Vector3,
        look_at: Vector3,
        up_vector: Vector3,
        vertical_fov: Degrees,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vertical_fov * f64::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).normalized();
        let u = up_vector.cross(&w);
        let v = w.cross(&u);

        let origin = look_from;

        Camera {
            lower_left_corner: origin
                - (u * (half_width * focus_distance))
                - (v * (half_height * focus_distance))
                - w * focus_distance,
            horizontal: u * (2. * half_width * focus_distance),
            vertical: v * (2. * half_height * focus_distance),
            origin,
            orthonormal_basis: Basis { u, v, w },
            lens_radius: aperture / 2.,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let random_dispersion = random_in_unit_sphere() * self.lens_radius;
        let offset = self.orthonormal_basis.u * random_dispersion.x
            + self.orthonormal_basis.v * random_dispersion.y;
        Ray::new(
            self.origin + offset,
            (self.lower_left_corner + self.horizontal * u + self.vertical * v)
                - self.origin
                - offset,
        )
    }
}
