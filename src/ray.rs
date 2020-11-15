use crate::vec3::Vec3;
/// Represents a Ray from the camera with an origin `A` and destination `B` in 3d.
pub struct Ray<'a> {
    A: &'a Vec3,
    B: &'a Vec3
}

/// Implements functionality related to a Ray.
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

    /// Calculates the point at the parameter T.
    /// The parameter T is the position along a line in 3D between ray origin and direction.
    /// it moves the point along the line.
    ///
    ///t=-2,-1,0,1,2,3,4
    /// <------====--->
    ///        A  B
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        return self.A + &(self.B * t);
    }
}
