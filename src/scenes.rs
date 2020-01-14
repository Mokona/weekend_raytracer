use crate::camera::Camera;
use crate::hit::{Hittable, HittableList, Sphere};
use crate::material::{DielectricParams, LambertianParams, Material, MetalParams};
use crate::vector3::Vector3;
use rand::Rng;

pub fn get_scene_1((width, height): (u32, u32)) -> (HittableList, Camera) {
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
        Vector3::from((-0.5, 0.15, -0.5)),
        0.15,
        Material::Dielectric(DielectricParams {
            refraction_index: 1.5,
        }),
    ));

    let sphere_6 = Box::new(Sphere::new(
        Vector3::from((0.3, -0.15, -0.5)),
        -0.20,
        Material::Dielectric(DielectricParams {
            refraction_index: 1.3,
        }),
    ));

    let world = HittableList::new(vec![
        sphere_1, sphere_2, sphere_3, sphere_4, sphere_5, sphere_6,
    ]);

    let look_from = Vector3::from((-2., 3., 1.5));
    let look_at = Vector3::from((0., 0., -1.));
    let up = Vector3::from((0., 1., 0.));

    let focus_distance = (look_from - look_at).norm();
    let aperture = 1.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        45.,
        width as f64 / height as f64,
        aperture,
        focus_distance,
    );

    (world, camera)
}

pub fn get_scene_2((width, height): (u32, u32)) -> (HittableList, Camera) {
    let mut spheres: Vec<Box<dyn Hittable>> = Vec::new();

    let ground_sphere = Box::new(Sphere::new(
        Vector3::from((0., -1000., 0.)),
        1000.,
        Material::Lambertian(LambertianParams {
            albedo: Vector3::from((0.5, 0.5, 0.5)),
        }),
    ));

    spheres.push(ground_sphere);

    let center = Vector3::from((4., 0.2, 0.));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let material_choice = rng.gen_range(0., 1.);

            let position = Vector3::from((
                a as f64 + rng.gen_range(0., 0.9),
                0.2,
                b as f64 + rng.gen_range(0., 0.9),
            ));

            let distance_from_center = (position - center).norm();

            if distance_from_center > 0.9 {
                let sphere = if material_choice < 0.8 {
                    Box::new(Sphere::new(
                        position,
                        0.2,
                        Material::Lambertian(LambertianParams {
                            albedo: Vector3::from((
                                rng.gen_range(0., 1.) * rng.gen_range(0., 1.),
                                rng.gen_range(0., 1.) * rng.gen_range(0., 1.),
                                rng.gen_range(0., 1.) * rng.gen_range(0., 1.),
                            )),
                        }),
                    ))
                } else if material_choice < 0.95 {
                    Box::new(Sphere::new(
                        position,
                        0.2,
                        Material::Metal(MetalParams {
                            albedo: Vector3::from((
                                0.5 * (1. + rng.gen_range(0., 1.)),
                                0.5 * (1. + rng.gen_range(0., 1.)),
                                0.5 * (1. + rng.gen_range(0., 1.)),
                            )),
                            fuzziness: 0.5 * rng.gen_range(0., 1.),
                        }),
                    ))
                } else {
                    Box::new(Sphere::new(
                        position,
                        0.2,
                        Material::Dielectric(DielectricParams {
                            refraction_index: 1.5,
                        }),
                    ))
                };
                spheres.push(sphere);
            }
        }
    }

    spheres.push(Box::new(Sphere::new(
        Vector3::from((-4., 1., 0.)),
        1.,
        Material::Lambertian(LambertianParams {
            albedo: Vector3::from((0.4, 0.2, 0.1)),
        }),
    )));

    spheres.push(Box::new(Sphere::new(
        Vector3::from((4., 1., 0.)),
        1.,
        Material::Metal(MetalParams {
            albedo: Vector3::from((0.8, 0.8, 0.2)),
            fuzziness: 0.,
        }),
    )));

    spheres.push(Box::new(Sphere::new(
        Vector3::from((0., 1., 0.)),
        1.,
        Material::Dielectric(DielectricParams {
            refraction_index: 1.5,
        }),
    )));

    let world = HittableList::new(spheres);

    let look_from = Vector3::from((-2., 1., 11.));
    let look_at = Vector3::from((0., 1., 0.));
    let up = Vector3::from((0., 1., 0.));

    let focus_distance = (look_from - look_at).norm();
    let aperture = 1.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up,
        30.,
        width as f64 / height as f64,
        aperture,
        focus_distance,
    );

    (world, camera)
}
