/// A list of hitable objects.

use crate::hitable;
use crate::ray;
use std::vec;

/// Represents a collection of geometries that can be hit by the ray.
pub struct HitableList {
    hitables: vec::Vec<Box<dyn hitable::Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            hitables: Vec::new()
        }
    }

    pub fn add(&mut self, hitable: Box<dyn hitable::Hitable>) {
        self.hitables.push(hitable);
    }
}

impl hitable::Hitable for HitableList {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, record: &mut hitable::HitRecord) -> bool {
        let mut temp_hit_record = hitable::HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hitable in &self.hitables {
            if hitable.hit(ray, t_min, closest_so_far, &mut temp_hit_record) {
                hit_anything = true;
                closest_so_far = temp_hit_record.t;

                record.t = temp_hit_record.t;
                record.point = temp_hit_record.point.clone();
                record.normal = temp_hit_record.normal.clone();
            }
        }
        return hit_anything;
    }

}
