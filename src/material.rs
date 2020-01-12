use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector3::{random_in_unit_sphere, Vector3};

type Attenuation = Vector3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(LambertianParams),
    Metal(MetalParams),
    Dielectric(DielectricParams),
}

#[derive(Copy, Clone)]
pub struct LambertianParams {
    pub albedo: Attenuation,
}

#[derive(Copy, Clone)]
pub struct MetalParams {
    pub albedo: Attenuation,
    pub fuzziness: f64,
}

#[derive(Copy, Clone)]
pub struct DielectricParams {
    pub refraction_index: f64,
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
        let reflected_fuzziness = if params.fuzziness > 0. {
            reflected + random_in_unit_sphere() * params.fuzziness.min(1.)
        } else {
            reflected
        };
        let reflected_ray = Ray::new(hit.point, reflected_fuzziness);

        Some((reflected_ray, params.albedo))
    } else {
        None
    }
}

fn refraction(incoming: Vector3, normal: Vector3, ni_over_nt: f64) -> Option<Vector3> {
    let incoming_normalized = incoming.normalized();
    let dt = incoming_normalized.dot(&normal);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        let refracted =
            (incoming_normalized - (normal * dt)) * ni_over_nt - (normal * discriminant.sqrt());
        Some(refracted)
    } else {
        None
    }
}

fn scatter_dielectric(
    ray: &Ray,
    hit: &HitRecord,
    params: DielectricParams,
) -> Option<(Ray, Vector3)> {
    let reflected = reflect(ray.direction, hit.normal);

    let (outward_normal, ni_overnt) = if ray.direction.dot(&hit.normal) > 0. {
        (-hit.normal, params.refraction_index)
    } else {
        (hit.normal, 1.0 / params.refraction_index)
    };

    match refraction(ray.direction, outward_normal, ni_overnt) {
        Some(refracted) => {
            let scattered = Ray::new(hit.point, refracted);
            let attenuation = Vector3::from((1., 1., 1.));
            Some((scattered, attenuation))
        }
        //        None => None, // Should reflect ?
        None => {
            let reflected_ray = Ray::new(hit.point, reflected);
            let attenuation = Vector3::from((1., 1., 1.));
            Some((reflected_ray, attenuation))
        }
    }
}

pub fn scatter(ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3)> {
    match hit.material {
        Material::Lambertian(params) => scatter_lambertian(ray, hit, params),
        Material::Metal(params) => scatter_metal(ray, hit, params),
        Material::Dielectric(params) => scatter_dielectric(ray, hit, params),
    }
}
