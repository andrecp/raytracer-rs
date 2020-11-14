use crate::vec3::Vec3;

pub struct Ray<'a> {
    A: &'a Vec3,
    B: &'a Vec3
}

impl<'a> Ray<'a> {
    pub fn new(A: &'a Vec3, B: &'a Vec3) -> Ray<'a> {
        Ray {
            A,
            B
        }
    }
    pub fn origin(&self) -> &Vec3 {
        return self.A;
    }

    pub fn direction(&self) -> &Vec3 {
        return self.B;
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        return self.A + &(self.B * t);
    }
}
