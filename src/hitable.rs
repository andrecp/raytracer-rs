/// Defines something that can be hit by a ray - something hitable.
///

use crate::vec3;
use crate::ray;

/// Records information of where a ray hit happened against an object.
pub struct HitRecord {
    pub t: f64,
    pub point: vec3::Vec3,
    pub normal: vec3::Vec3
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            point: vec3::Vec3::zero(),
            normal: vec3::Vec3::zero(),
        }
    }
}

/// Geometries that implement this trait can be hit by a Ray Tracer.
pub trait Hitable {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
