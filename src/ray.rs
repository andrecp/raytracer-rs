use crate::vec3::Vec3;
/// Represents a Ray from the camera with an origin `A` and destination `B` in 3d.
pub struct Ray<'a> {
    A: &'a Vec3,
    B: &'a Vec3
}

/// Implements functionality relayed to a Ray.
impl<'a> Ray<'a> {

    /// Creates a new Ray between points A and B.
    pub fn new(A: &'a Vec3, B: &'a Vec3) -> Ray<'a> {
        Ray {
            A,
            B
        }
    }

    /// Returns the Origin of the ray.
    pub fn origin(&self) -> &Vec3 {
        return self.A;
    }

    /// Returns the Direction of the ray.
    pub fn direction(&self) -> &Vec3 {
        return self.B;
    }

    /// Calculates the point at the parameter.
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        return self.A + &(self.B * t);
    }
}
