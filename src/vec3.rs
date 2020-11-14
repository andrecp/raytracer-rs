use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign, Index, IndexMut, Neg};


pub struct RGB {
    data: Vec3
}

impl RGB {
    pub fn zero() -> RGB {
        RGB {
            data: Vec3::zero()
        }
    }

    pub fn new(v0: f64, v1: f64, v2: f64) -> RGB {
        RGB {
            data: Vec3::new(v0, v1, v2)
        }
    }

    pub fn r(&self) -> f64 {
        return self.data[0]
    }

    pub fn g(&self) -> f64 {
        return self.data[1]
    }

    pub fn b(&self) -> f64 {
        return self.data[2]
    }
}

pub struct XYZ {
    data: Vec3
}

impl XYZ {
    pub fn zero() -> XYZ {
        XYZ {
            data: Vec3::zero()
        }
    }

    pub fn new(vec: Vec3) -> XYZ {
        XYZ {
            data: vec
        }
    }


    pub fn x(&self) -> f64 {
        return self.data[0]
    }

    pub fn y(&self) -> f64 {
        return self.data[1]
    }

    pub fn z(&self) -> f64 {
        return self.data[2]
    }
}
pub struct Vec3 {
    vec3: Vec<f64>
}

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    return vec / vec.vec3.len() as f64;
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        let mut v = Vec::with_capacity(3);
        v.push(0.0);
        v.push(0.0);
        v.push(0.0);
        Vec3 {
            vec3: v
        }
    }

    pub fn new(v0: f64, v1: f64, v2: f64) -> Vec3 {
        let mut v = Vec::with_capacity(3);
        v.push(v0);
        v.push(v1);
        v.push(v2);
        Vec3 {
            vec3: v
        }
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).sqrt();
        self[0] *= k;
        self[1] *= k;
        self[2] *= k;
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        return self[0] * other[0] + self[1] * other[1] + self[2] * other[2];
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        return Vec3::new(
            self[1] * other[2] - self[2] * other[1],
            -(self[0] * other[2] - self[2] * other[0]),
            self[0] * other[1] - self[1] * other[0],
        )
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3::new(-self[0], -self[1], -self[2]);
    }
}

// Vec3[0]
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        return &self.vec3[idx];
    }
}

impl IndexMut<usize> for Vec3 {

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        return &mut self.vec3[idx];
    }
}

// Vec3 =+ Vec2
impl AddAssign for Vec3 {

    fn add_assign(&mut self, other: Self) {
        self.vec3[0] = self[0] + other[0];
        self.vec3[1] = self[1] + other[1];
        self.vec3[2] = self[2] + other[2];
    }
}

// Vec3 =+ 3
impl AddAssign<f64> for Vec3 {

    fn add_assign(&mut self, other: f64) {
        self.vec3[0] = self[0] + other;
        self.vec3[1] = self[1] + other;
        self.vec3[2] = self[2] + other;
    }
}

// Vec3 + Vec2
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Vec3{
        return Vec3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2]);
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Vec3 {
        return Vec3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2]);
    }
}

// Vec3 + 3
impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Vec3{
        return Vec3::new(self[0] + other, self[1] + other, self[2] + other);
    }
}

// Vec 3 *= Vec2
impl MulAssign for Vec3 {

    fn mul_assign(&mut self, other: Self) {
        self.vec3[0] = self[0] * other[0];
        self.vec3[1] = self[1] * other[1];
        self.vec3[2] = self[2] * other[2];
    }
}

// Vec 3 *= 4
impl MulAssign<f64> for Vec3 {

    fn mul_assign(&mut self, other: f64) {
        self.vec3[0] = self[0] * other;
        self.vec3[1] = self[1] * other;
        self.vec3[2] = self[2] * other;
    }
}

// Vec 3 = Vec2 * 4
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self[0] * other, self[1] * other, self[2] * other)
    }
}

// Vec 3 = Vec2 * 4
impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self[0] * other, self[1] * other, self[2] * other)
    }
}

// Vec 3 = Vec2 * Vec3
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

// Vec 3 = Vec3 / 4
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self[0] / other, self[1] / other, self[2] / other)
    }
}

// Vec 3 = Vec3 / 4
impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self[0] / other, self[1] / other, self[2] / other)
    }
}

// Vec 3 = Vec3 / Vec4
impl Div<&Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: &Vec3) -> Vec3 {
        Vec3::new(self[0] / other[0], self[1] / other[1], self[2] / other[2])
    }
}

// Vec 3 = Vec3 / Vec4
impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] / other[0], self[1] / other[1], self[2] / other[2])
    }
}

// Vec 3 /= 3
impl DivAssign<&f64> for Vec3 {

    fn div_assign(&mut self, other: &f64) {
        self.vec3[0] = self[0] / other;
        self.vec3[1] = self[1] / other;
        self.vec3[2] = self[2] / other;
    }
}

// Vec 3 /= 3
impl DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, other: f64) {
        self.vec3[0] = self[0] / other;
        self.vec3[1] = self[1] / other;
        self.vec3[2] = self[2] / other;
    }
}

// Vec 3 /= Vec2
impl DivAssign for Vec3 {

    fn div_assign(&mut self, other: Self) {
        self.vec3[0] = self[0] / other[0];
        self.vec3[1] = self[1] / other[1];
        self.vec3[2] = self[2] / other[2];
    }
}


// Vec 3 -= 3
impl SubAssign<f64> for Vec3 {

    fn sub_assign(&mut self, other: f64) {
        self.vec3[0] = self[0] - other;
        self.vec3[1] = self[1] - other;
        self.vec3[2] = self[2] - other;
    }
}

// Vec 3 -= Vec2
impl SubAssign for Vec3 {

    fn sub_assign(&mut self, other: Self) {
        self.vec3[0] = self[0] - other[0];
        self.vec3[1] = self[1] - other[1];
        self.vec3[2] = self[2] - other[2];
    }
}

// Vec 3 - 3
impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Vec3 {
        return Vec3::new(self[0] - other, self[1] - other, self[2] - other);
    }
}

// Vec 3 - Vec2
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        return Vec3::new(self[0] - other[0], self[1] - other[1], self[2] - other[2]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vec_works() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);

        let v = Vec3::zero();
        assert_eq!(v[0], 0.0);
        assert_eq!(v[1], 0.0);
        assert_eq!(v[2], 0.0);
    }

    #[test]
    fn add_assign_works() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        v1 += v2;

        assert_eq!(v1[0], 3.0);
        assert_eq!(v1[1], 5.0);
        assert_eq!(v1[2], 7.0);

        v1 += 1.0;
        assert_eq!(v1[0], 4.0);
        assert_eq!(v1[1], 6.0);
        assert_eq!(v1[2], 8.0);
    }

    #[test]
    fn mult_assign_works() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        v1 *= v2;

        assert_eq!(v1[0], 2.0);
        assert_eq!(v1[1], 6.0);
        assert_eq!(v1[2], 12.0);

        v1 *= 2.0;
        assert_eq!(v1[0], 4.0);
        assert_eq!(v1[1], 12.0);
        assert_eq!(v1[2], 24.0);
    }

    #[test]
    fn div_assign_works() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        v1 /= v2;

        assert_eq!(v1[0], 1.0/2.0);
        assert_eq!(v1[1], 2.0/3.0);
        assert_eq!(v1[2], 3.0/4.0);

        v1 /= 0.5;

        assert_eq!(v1[0], 2.0/2.0);
        assert_eq!(v1[1], 4.0/3.0);
        assert_eq!(v1[2], 6.0/4.0);
    }
}
