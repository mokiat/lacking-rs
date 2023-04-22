#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn basis_x() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }

    pub fn basis_y() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    pub fn basis_z() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;
    use assert_approx_eq::assert_approx_eq;

    pub fn assert_vec3_eq(vec: Vec3, x: f64, y: f64, z: f64) {
        assert_approx_eq!(vec.x, x);
        assert_approx_eq!(vec.y, y);
        assert_approx_eq!(vec.z, z);
    }

    #[test]
    fn test_new() {
        let new = Vec3::new(1.0, 2.0, 3.0);
        assert_vec3_eq(new, 1.0, 2.0, 3.0);
    }

    #[test]
    fn test_zero() {
        let zero = Vec3::zero();
        assert_vec3_eq(zero, 0.0, 0.0, 0.0);
    }

    #[test]
    fn test_basis_x() {
        let basis = Vec3::basis_x();
        assert_vec3_eq(basis, 1.0, 0.0, 0.0);
    }

    #[test]
    fn test_basis_y() {
        let basis = Vec3::basis_y();
        assert_vec3_eq(basis, 0.0, 1.0, 0.0);
    }

    #[test]
    fn test_basis_z() {
        let basis = Vec3::basis_z();
        assert_vec3_eq(basis, 0.0, 0.0, 1.0);
    }
}
