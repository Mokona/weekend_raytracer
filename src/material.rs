use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector3::{random_in_unit_sphere, Vector3};

type Attenuation = Vector3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(LambertianParams),
}

#[derive(Copy, Clone)]
pub struct LambertianParams {
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

pub fn scatter(ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3)> {
    match hit.material {
        Material::Lambertian(params) => scatter_lambertian(ray, hit, params),
    }
}
