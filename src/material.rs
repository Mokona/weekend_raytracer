use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector3::{random_in_unit_sphere, Vector3};

type Attenuation = Vector3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(LambertianParams),
    Metal(MetalParams),
}

#[derive(Copy, Clone)]
pub struct LambertianParams {
    pub albedo: Attenuation,
}

#[derive(Copy, Clone)]
pub struct MetalParams {
    pub albedo: Attenuation,
}

fn scatter_lambertian(
    _ray: &Ray,
    hit: &HitRecord,
    params: LambertianParams,
) -> Option<(Ray, Vector3)> {
    let diffuse_direction = hit.point + hit.normal + random_in_unit_sphere();
    let diffuse_ray = Ray::new(hit.point, diffuse_direction - hit.point);

    Some((diffuse_ray, params.albedo))
}

fn reflect(incoming: Vector3, normal: Vector3) -> Vector3 {
    incoming - normal * 2. * incoming.dot(&normal)
}

fn scatter_metal(ray: &Ray, hit: &HitRecord, params: MetalParams) -> Option<(Ray, Vector3)> {
    let reflected = reflect(ray.direction.normalized(), hit.normal);

    if reflected.dot(&hit.normal) > 0. {
        let reflected_ray = Ray::new(hit.point, reflected);

        Some((reflected_ray, params.albedo))
    } else {
        None
    }
}

pub fn scatter(ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3)> {
    match hit.material {
        Material::Lambertian(params) => scatter_lambertian(ray, hit, params),
        Material::Metal(params) => scatter_metal(ray, hit, params),
    }
}
