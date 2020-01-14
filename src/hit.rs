use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vector3,
    pub normal: Vector3,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.list
            .iter()
            .fold((t_max, None), |(closest_t, current_hit), b| {
                if let Some(new_hit) = b.hit(&ray, t_min, closest_t) {
                    (new_hit.t, Some(new_hit))
                } else {
                    (closest_t, current_hit)
                }
            })
            .1
    }
}

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Sphere {
    fn get_hit_in_range(&self, ray: &Ray, t_min: f64, t_max: f64, hit: f64) -> Option<HitRecord> {
        if t_min < hit && hit < t_max {
            let hit_point = ray.point_at_parameter(hit);
            Some(HitRecord {
                t: hit,
                point: hit_point,
                normal: (hit_point - self.center) / self.radius,
                material: &(*self.material),
            })
        } else {
            None
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let sphere_to_ray_origin = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = sphere_to_ray_origin.dot(&ray.direction);
        let c = sphere_to_ray_origin.dot(&sphere_to_ray_origin) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant < 0. {
            None
        } else {
            let first_hit = (-b - discriminant.sqrt()) / a;
            self.get_hit_in_range(ray, t_min, t_max, first_hit).or({
                let second_hit = (-b + discriminant.sqrt()) / a;
                self.get_hit_in_range(ray, t_min, t_max, second_hit)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;

    fn get_dummy_material() -> Box<dyn Material> {
        Box::new(Lambertian {
            albedo: Vector3::default(),
        })
    }

    #[test]
    fn hits_sphere_from_origin() {
        let sphere = Sphere::new(Vector3::from((0., 0., -2.)), 1., get_dummy_material());
        let ray = Ray::new(Vector3::default(), Vector3::from((0., 0., -1.)));

        let hit = sphere.hit(&ray, 0., 2.).unwrap();

        assert_eq!(Vector3::from((0., 0., -1.)), hit.point);
        assert_eq!(Vector3::from((0., 0., 1.)), hit.normal);
    }

    #[test]
    fn hits_sphere_from_origin_skipping_first_hit() {
        let sphere = Sphere::new(Vector3::from((0., 0., -2.)), 1., get_dummy_material());
        let ray = Ray::new(Vector3::default(), Vector3::from((0., 0., -1.)));

        let hit = sphere.hit(&ray, 2., 4.).unwrap();

        assert_eq!(Vector3::from((0., 0., -3.)), hit.point);
        assert_eq!(Vector3::from((0., 0., -1.)), hit.normal);
    }

    #[test]
    fn hits_sphere_from_origin_skipping_two_hits() {
        let sphere = Sphere::new(Vector3::from((0., 0., -2.)), 1., get_dummy_material());
        let ray = Ray::new(Vector3::default(), Vector3::from((0., 0., -1.)));

        let hit = sphere.hit(&ray, 4., 10.);
        assert!(hit.is_none());
    }
}
