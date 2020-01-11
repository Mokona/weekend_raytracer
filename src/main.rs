mod color;
mod ppm;
mod ray;
mod vector3;

use color::Color;
use ray::Ray;
use vector3::Vector3;

fn color(ray: Ray) -> Color {
    let mut normalized_direction = ray.point_at_parameter(1.);
    normalized_direction.normalize();

    let t = (0.5 * (normalized_direction.y + 1.)) as f32;

    let white = Color::new(255, 255, 255);
    let blue = Color::new(127, 178, 255);

    white.lerp(blue, t)
}

fn main() {
    let width = 200;
    let height = 100;

    let lower_left_corner = Vector3::from((-2., -1., -1.));
    let horizontal = Vector3::from((4., 0., 0.));
    let vertical = Vector3::from((0., 2., 0.));
    let origin = Vector3::new();

    let output = ppm::get_file_content(width, height, |x: u32, y: u32| -> Color {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;

        let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

        color(ray)
    });

    print!("{}", output);
}
