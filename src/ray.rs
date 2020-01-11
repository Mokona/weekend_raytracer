use crate::vector3::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3::Vector3;

    #[test]
    fn computes_point_at_parameter() {
        let origin = Vector3::from((0., 0., 0.));
        let direction = Vector3::from((0., 1., 0.));
        let ray = Ray::new(origin, direction);

        let result = Vector3::from((0., 4., 0.));

        assert_eq!(result, ray.point_at_parameter(4.));
    }
}
