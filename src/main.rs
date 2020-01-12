mod camera;
mod color;
mod hit;
mod ppm;
mod ray;
mod vector3;

use color::Color;
use hit::{Hittable, HittableList, Sphere};
use ray::Ray;
use std::f64;
use vector3::Vector3;

fn color(ray: Ray, world: &HittableList) -> Color {
    let hit_point = world.hit(&ray, 0.0, f64::MAX);
    match hit_point {
        Some(hit) => {
            let half_normal = (hit.normal + Vector3::from((1., 1., 1.))) * 0.5;
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

    let sphere_1 = Box::new(Sphere::new(Vector3::from((0., 0., -1.)), 0.5));
    let sphere_2 = Box::new(Sphere::new(Vector3::from((0., -100.5, -1.)), 100.));

    let world = HittableList::new(vec![sphere_1, sphere_2]);
    let camera = camera::Camera::new();

    let output = ppm::get_file_content(width, height, |x: u32, y: u32| -> Color {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;

        let ray = camera.get_ray(u, v);

        color(ray, &world)
    });

    print!("{}", output);
}
