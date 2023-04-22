use std::f64::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct Angle {
    amount: f64,
}

impl Angle {
    pub fn from_radians(radians: f64) -> Angle {
        Angle { amount: radians }
    }

    pub fn from_degrees(degrees: f64) -> Angle {
        Angle {
            amount: (degrees * PI) / 180.0,
        }
    }

    pub fn to_degrees(&self) -> f64 {
        self.amount * (180.0 / PI)
    }

    pub fn to_radians(&self) -> f64 {
        self.amount
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use std::f64::consts::PI;

    #[test]
    fn construct_from_degrees() {
        let flat = Angle::from_degrees(0.0);
        assert_approx_eq!(flat.to_degrees(), 0.0);
        assert_approx_eq!(flat.to_radians(), 0.0);

        let inclined = Angle::from_degrees(45.0);
        assert_approx_eq!(inclined.to_degrees(), 45.0);
        assert_approx_eq!(inclined.to_radians(), PI / 4.0);

        let vertical = Angle::from_degrees(90.0);
        assert_approx_eq!(vertical.to_degrees(), 90.0);
        assert_approx_eq!(vertical.to_radians(), PI / 2.0);
    }

    #[test]
    fn construct_from_radians() {
        let flat = Angle::from_radians(0.0);
        assert_approx_eq!(flat.to_degrees(), 0.0);
        assert_approx_eq!(flat.to_radians(), 0.0);

        let inclined = Angle::from_radians(PI / 4.0);
        assert_approx_eq!(inclined.to_degrees(), 45.0);
        assert_approx_eq!(inclined.to_radians(), PI / 4.0);

        let vertical = Angle::from_radians(PI / 2.0);
        assert_approx_eq!(vertical.to_degrees(), 90.0);
        assert_approx_eq!(vertical.to_radians(), PI / 2.0);
    }
}
