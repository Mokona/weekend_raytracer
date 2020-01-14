use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector3::{random_in_unit_sphere, Vector3};
use rand::Rng;

type Attenuation = Vector3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3)>;
}

pub struct Lambertian {
    pub albedo: Attenuation,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3)> {
        let diffuse_direction = hit.point + hit.normal + random_in_unit_sphere();
        let diffuse_ray = Ray::new(hit.point, diffuse_direction - hit.point);

        Some((diffuse_ray, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Attenuation,
    pub fuzziness: f64,
}

fn reflect(incoming: Vector3, normal: Vector3) -> Vector3 {
    incoming - normal * 2. * incoming.dot(&normal)
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3)> {
        let reflected = reflect(ray.direction.normalized(), hit.normal);

        if reflected.dot(&hit.normal) > 0. {
            let reflected_fuzziness = if self.fuzziness > 0. {
                reflected + random_in_unit_sphere() * self.fuzziness.min(1.)
            } else {
                reflected
            };
            let reflected_ray = Ray::new(hit.point, reflected_fuzziness);

            Some((reflected_ray, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
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

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3)> {
        let reflected = reflect(ray.direction, hit.normal);

        let (outward_normal, ni_overnt, cosine) = if ray.direction.dot(&hit.normal) > 0. {
            let cosine =
                self.refraction_index * ray.direction.dot(&hit.normal) / ray.direction.norm();
            (-hit.normal, self.refraction_index, cosine)
        } else {
            let cosine = -ray.direction.dot(&hit.normal) / ray.direction.norm();
            (hit.normal, 1.0 / self.refraction_index, cosine)
        };

        let (reflection_probability, refracted) =
            match refraction(ray.direction, outward_normal, ni_overnt) {
                Some(refracted) => (schlick(cosine, self.refraction_index), refracted),
                None => (1., Vector3::default()),
            };

        let attenuation = Vector3::from((1., 1., 1.));
        if rand::thread_rng().gen_range(0., 1.) < reflection_probability {
            Some((Ray::new(hit.point, reflected), attenuation))
        } else {
            Some((Ray::new(hit.point, refracted), attenuation))
        }
    }
}
