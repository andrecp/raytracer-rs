use crate::vec3;
use crate::hitable;
use crate::ray;

/// Represents a sphere in 3D with a center and a radius.
pub struct Sphere {
    center: vec3::XYZ,
    radius: f64
}

/// Implements sphere functionality
impl Sphere {
    pub fn new(center: vec3::XYZ, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

/// Verifies if a Sphere is going to be hit by a given Ray.
///
/// Calculates the Normal (vector perpendicular to the surface), for the sphere this is in the direction
/// of the hitpoint minus the center.
///
/// Calculates the point of intersection.
impl hitable::Hitable for Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, record: &mut hitable::HitRecord) -> bool {
        let ray_direction = ray.direction();
        let oc = ray.origin() - &self.center.vec3();
        let a = ray_direction.dot(ray_direction);
        let b = ray_direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            // Inside the hitbox.
            if (temp < t_max) && (temp > t_min) {
                record.t = temp;
                record.point = ray.point_at_parameter(temp);
                record.normal = (&record.point - self.center.vec3()) / self.radius();
                return true;
            }
            let temp = (-b + discriminant.sqrt()) / a;
            // Inside the hitbox.
            if (temp < t_max) && (temp > t_min) {
                record.t = temp;
                record.point = ray.point_at_parameter(temp);
                record.normal = (&record.point - self.center.vec3()) / self.radius();
                return true;
            }
        }
        return false;
    }

}
