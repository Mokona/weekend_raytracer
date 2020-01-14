use crate::camera::Camera;
use crate::hit::{Hittable, HittableList, Sphere};
use crate::material::{DielectricParams, LambertianParams, Material, MetalParams};
use crate::vector3::Vector3;

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
