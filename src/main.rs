mod camera;
mod color;
mod hit;
mod ppm;
mod ray;
mod vector3;

use color::Color;
use hit::{Hittable, HittableList, Sphere};
use rand::Rng;
use ray::Ray;
use std::f64;
use vector3::Vector3;

fn random_in_unit_sphere() -> Vector3 {
    use std::iter;

    let mut rng = rand::thread_rng();

    let in_unit_coordinates: (f64, f64, f64) = iter::repeat_with(|| {
        (
            rng.gen_range(0., 1.),
            rng.gen_range(0., 1.),
            rng.gen_range(0., 1.),
        )
    })
    .skip_while(|(x, y, z)| Vector3::from((*x, *y, *z)).squared_norm() > 1.)
    .next()
    .unwrap();
    Vector3::from(in_unit_coordinates)
}

fn color(ray: Ray, world: &HittableList) -> Color {
    let hit_point = world.hit(&ray, 0.0, f64::MAX);
    match hit_point {
        Some(hit) => {
            let diffuse_direction = hit.point + hit.normal + random_in_unit_sphere();
            let diffuse_ray = Ray::new(hit.point, diffuse_direction - hit.point);
            let (r, g, b): (f64, f64, f64) = color(diffuse_ray, world).into();
            Color::from((r / 2. / 255., g / 2. / 255., b / 2. / 255.))
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

    let sphere_1 = Box::new(Sphere::new(Vector3::from((0., 0., -1.)), 0.5));
    let sphere_2 = Box::new(Sphere::new(Vector3::from((0., -100.5, -1.)), 100.));

    let world = HittableList::new(vec![sphere_1, sphere_2]);
    let camera = camera::Camera::new();

    let output = ppm::get_file_content(width, height, |x: u32, y: u32| -> Color {
        let mut rng = rand::thread_rng();
        let mut color_accumulator = Vector3::default();

        for _i in 0..sub_sample_count {
            let u = (x as f64 + rng.gen_range(0., 1.)) / width as f64;
            let v = (y as f64 + rng.gen_range(0., 1.)) / height as f64;

            let ray = camera.get_ray(u, v);
            let color = color(ray, &world);
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

#[cfg(test)]
mod tests {
    use crate::random_in_unit_sphere;

    #[test]
    fn test_random_unit_vector() {
        for _i in 0..100 {
            let random_unit = random_in_unit_sphere();
            assert!(random_unit.squared_norm() <= 1.);
        }
    }
}
