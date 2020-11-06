use std::ops::{Sub, Index, Neg, AddAssign};

struct Vec3 {
    vec3: Vec<f64>
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

impl AddAssign for Vec3 {

    fn add_assign(&mut self, other: Self) {
        self.vec3[0] = self[0] + other[0];
        self.vec3[1] = self[1] + other[1];
        self.vec3[2] = self[2] + other[2];
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
    }
}
