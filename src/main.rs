mod color;
mod ppm;
mod ray;
mod vector3;

use color::Color;
use ray::Ray;
use vector3::Vector3;

fn hit_sphere(center: Vector3, radius: f64, ray: &Ray) -> Option<f64> {
    let sphere_to_ray_origin = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2. * sphere_to_ray_origin.dot(&ray.direction);
    let c = sphere_to_ray_origin.dot(&sphere_to_ray_origin) - radius * radius;
    let discriminant = b * b - 4. * a * c;

    if discriminant < 0. {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2. * a))
    }
}

fn color(ray: Ray) -> Color {
    let sphere_center = Vector3::from((0., 0., -1.));
    let hit_point = hit_sphere(sphere_center, 0.5, &ray);
    match hit_point {
        Some(t) => {
            let normal = (ray.point_at_parameter(t) - sphere_center).normalized();
            let half_normal = (normal + Vector3::from((1., 1., 1.))) * 0.5;
            Color::from((half_normal.x, half_normal.y, half_normal.z))
        }
        None => {
            let normalized_direction = ray.point_at_parameter(1.).normalized();
            let t = (0.5 * (normalized_direction.y + 1.)) as f32;

            let white = Color::new(255, 255, 255);
            let blue = Color::new(127, 178, 255);

            white.lerp(blue, t)
        }
    }
}

fn main() {
    let width = 400;
    let height = 200;

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
