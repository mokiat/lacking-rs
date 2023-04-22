use crate::math;

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    pub fn to_vec3(&self) -> math::Vec3 {
        math::Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::math::vec3::tests::assert_vec3_eq;

    #[test]
    fn test_to_vec3() {
        let vec = Vec4 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let vec3 = vec.to_vec3();
        assert_vec3_eq(vec3, 1.0, 2.0, 3.0);
    }
}
