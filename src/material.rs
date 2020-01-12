use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector3::{random_in_unit_sphere, Vector3};
use rand::Rng;

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

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0_root = (1. - refraction_index) / (1. + refraction_index);
    let r0 = r0_root * r0_root;
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

fn scatter_dielectric(
    ray: &Ray,
    hit: &HitRecord,
    params: DielectricParams,
) -> Option<(Ray, Vector3)> {
    let reflected = reflect(ray.direction, hit.normal);

    let (outward_normal, ni_overnt, cosine) = if ray.direction.dot(&hit.normal) > 0. {
        let cosine =
            params.refraction_index * ray.direction.dot(&hit.normal) / ray.direction.norm();
        (-hit.normal, params.refraction_index, cosine)
    } else {
        let cosine = -ray.direction.dot(&hit.normal) / ray.direction.norm();
        (hit.normal, 1.0 / params.refraction_index, cosine)
    };

    let (reflection_probability, refracted) =
        match refraction(ray.direction, outward_normal, ni_overnt) {
            Some(refracted) => (schlick(cosine, params.refraction_index), refracted),
            None => (1., Vector3::default()),
        };

    let attenuation = Vector3::from((1., 1., 1.));
    if rand::thread_rng().gen_range(0., 1.) < reflection_probability {
        Some((Ray::new(hit.point, reflected), attenuation))
    } else {
        Some((Ray::new(hit.point, refracted), attenuation))
    }
}

pub fn scatter(ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3)> {
    match hit.material {
        Material::Lambertian(params) => scatter_lambertian(ray, hit, params),
        Material::Metal(params) => scatter_metal(ray, hit, params),
        Material::Dielectric(params) => scatter_dielectric(ray, hit, params),
    }
}
