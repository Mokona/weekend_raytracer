use crate::vector3::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Vector3 {
        let mut v = self.direction.clone();
        v *= t;
        v += self.origin.clone();
        v
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
        let ray = Ray { origin, direction };

        let result = Vector3::from((0., 4., 0.));

        assert_eq!(result, ray.point_at_parameter(4.));
    }
}
