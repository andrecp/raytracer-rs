use crate::vec3;
use crate::ray;

pub struct Camera {
    origin: vec3::XYZ,
    lower_left_corner: vec3::XYZ,
    horizontal: vec3::XYZ,
    vertical: vec3::XYZ,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: vec3::XYZ::new_x_y_z(0.0, 0.0, 0.0),
            lower_left_corner: vec3::XYZ::new_x_y_z(-2.0, -1.0, -1.0),
            horizontal: vec3::XYZ::new_x_y_z(4.0, 0.0, 0.0),
            vertical: vec3::XYZ::new_x_y_z(0.0, 2.0, 0.0)
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        let direction = (self.lower_left_corner.vec3() + &(self.horizontal.vec3() * u)) + self.vertical.vec3() * v;
        return ray::Ray::new(
            self.origin.vec3(),
            direction
        )

    }
}
