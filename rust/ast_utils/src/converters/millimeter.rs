use super::*;

pub struct Millimeter {
    value: f64,
    unit: DistanceUnit,
}

impl Millimeter {
    pub fn new(value: f64) -> Millimeter {
        Millimeter {
            value: value,
            unit: DistanceUnit::Millimeter,
        }
    }
}

impl HasConvertableUnit for Millimeter {
    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &DistanceUnit {
        &self.unit
    }

    fn convert_scalar<'a>(&self, to_unit: &DistanceUnit) -> Result<f64, &'a str> {
        match to_unit {
            DistanceUnit::Millimeter => Ok(self.value),
            DistanceUnit::Centimeter => Ok(self.value.pow10(-1)),
            DistanceUnit::Meter => Ok(self.value.pow10(-3)),
            DistanceUnit::Kilometer => Ok(self.value.pow10(-6)),
            _ => Err("Millimeter misses some distances"),
        }
    }
}

mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_0millimeters_to_centimeters() {
        let res = Millimeter::new(0.0).convert_scalar(&DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(0.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_millimeter() {
        let res = Millimeter::new(1.0).convert_scalar(&DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_centimeter() {
        let res = Millimeter::new(10.0).convert_scalar(&DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_meter() {
        let res = Millimeter::new(1000.0).convert_scalar(&DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_millimeters_to_kilometer() {
        let res = Millimeter::new(1_000_000.0).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }
}
