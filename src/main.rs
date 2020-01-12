mod camera;
mod color;
mod hit;
mod material;
mod ppm;
mod ray;
mod vector3;

use color::Color;
use hit::{Hittable, HittableList, Sphere};
use material::{DielectricParams, LambertianParams, Material, MetalParams};
use rand::Rng;
use ray::Ray;
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

fn main() {
    let width = 400;
    let height = 200;
    let sub_sample_count = 100;

    let sphere_1 = Box::new(Sphere::new(
        Vector3::from((0., 0., -1.)),
        0.5,
        Material::Lambertian(LambertianParams {
            albedo: Vector3::from((0.2, 1., 0.2)),
        }),
    ));
    let sphere_2 = Box::new(Sphere::new(
        Vector3::from((0., -100.5, -1.)),
        100.,
        Material::Lambertian(LambertianParams {
            albedo: Vector3::from((0.5, 0.5, 0.5)),
        }),
    ));

    let sphere_3 = Box::new(Sphere::new(
        Vector3::from((1.3, 0., -1.)),
        0.5,
        Material::Metal(MetalParams {
            albedo: Vector3::from((0.8, 0.6, 0.2)),
            fuzziness: 0.3,
        }),
    ));

    let sphere_4 = Box::new(Sphere::new(
        Vector3::from((-1.3, 0., -1.)),
        0.5,
        Material::Metal(MetalParams {
            albedo: Vector3::from((0.8, 0.8, 0.2)),
            fuzziness: 1.,
        }),
    ));

    let sphere_5 = Box::new(Sphere::new(
        Vector3::from((-0.3, 0.15, -0.5)),
        0.15,
        Material::Dielectric(DielectricParams {
            refraction_index: 1.5,
        }),
    ));

    let sphere_6 = Box::new(Sphere::new(
        Vector3::from((0.3, -0.15, -0.5)),
        0.15,
        Material::Dielectric(DielectricParams {
            refraction_index: 1.5,
        }),
    ));

    let world = HittableList::new(vec![
        sphere_1, sphere_2, sphere_3, sphere_4, sphere_5, sphere_6,
    ]);
    let camera = camera::Camera::new();

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
