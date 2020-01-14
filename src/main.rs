mod camera;
mod color;
mod hit;
mod material;
mod ppm;
mod ray;
mod scenes;
mod vector3;

use camera::Camera;
use color::Color;
use hit::{Hittable, HittableList};
use rand::Rng;
use ray::Ray;
use scenes::{get_scene_1, get_scene_2};
use std::f64;
use vector3::Vector3;

const MAX_DEPTH_LIMIT: u32 = 50;

fn color(ray: Ray, world: &HittableList, depth_limit: u32) -> Color {
    if depth_limit >= MAX_DEPTH_LIMIT {
        return Color::default();
    }

    let hit_point = world.hit(&ray, 0.001, f64::MAX);
    match hit_point {
        Some(hit) => {
            if let Some((reflection_ray, attenuation)) = material::scatter(&ray, &hit) {
                let (r, g, b): (f64, f64, f64) =
                    color(reflection_ray, world, depth_limit + 1).into();
                Color::from((
                    attenuation.x * r / 255.,
                    attenuation.y * g / 255.,
                    attenuation.z * b / 255.,
                ))
            } else {
                Color::default()
            }
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

fn simple_gamma_on_component(component: f64) -> f64 {
    assert!(component <= 255.);
    (component / 255.).sqrt()
}

fn simple_gamma_correction(color: Color) -> Color {
    let color_as_tuple: (f64, f64, f64) = color.into();
    let fixed_tuple = (
        simple_gamma_on_component(color_as_tuple.0),
        simple_gamma_on_component(color_as_tuple.1),
        simple_gamma_on_component(color_as_tuple.2),
    );

    Color::from(fixed_tuple)
}

enum Scene {
    Scene1,
    Scene2,
}

fn get_scene(scene: Scene, geometry: (u32, u32)) -> (HittableList, Camera) {
    match scene {
        Scene::Scene1 => get_scene_1(geometry),
        Scene::Scene2 => get_scene_2(geometry),
    }
}

fn main() {
    let width = 400;
    let height = 200;
    let sub_sample_count = 100;

    let (world, camera) = get_scene(Scene::Scene2, (width, height));

    let output = ppm::get_file_content(width, height, |x: u32, y: u32| -> Color {
        let mut rng = rand::thread_rng();
        let mut color_accumulator = Vector3::default();

        for _i in 0..sub_sample_count {
            let u = (x as f64 + rng.gen_range(0., 1.)) / width as f64;
            let v = (y as f64 + rng.gen_range(0., 1.)) / height as f64;

            let ray = camera.get_ray(u, v);
            let color = color(ray, &world, 0);
            let color_as_tuple: (f64, f64, f64) = color.into();
            color_accumulator += Vector3::from(color_as_tuple);
        }

        color_accumulator /= sub_sample_count as f64;
        color_accumulator /= 255.;

        simple_gamma_correction(Color::from((
            color_accumulator.x,
            color_accumulator.y,
            color_accumulator.z,
        )))
    });

    print!("{}", output);
}
