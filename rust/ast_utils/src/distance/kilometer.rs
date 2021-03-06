use super::*;

pub struct Kilometer {
    value: f64,
}

impl Kilometer {
    pub fn new(value: f64) -> Kilometer {
        Kilometer { value }
    }
}

impl HasConvertableUnit for Kilometer {
    type Unit = DistanceUnit;

    fn scalar(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> &Self::Unit {
        &DistanceUnit::Kilometer
    }

    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str> {
        let value = self.scalar();

        if value == 0.0 {
            return Ok(0.0);
        }

        match to_unit {
            DistanceUnit::Millimeter => Ok(value.pow10(6)),
            DistanceUnit::Centimeter => Ok(value.pow10(5)),
            DistanceUnit::Meter => Ok(value.pow10(3)),
            DistanceUnit::Kilometer => Ok(value),
            DistanceUnit::Mile => Ok(value / 1.609_344),
            DistanceUnit::AstronomicalUnit => Ok(value * 6.685e-9),
            _ => Err("Can not convert from Kilometer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_kilometers_to_millimeter() {
        let res = Kilometer::new(1.0e-6).convert_scalar(&DistanceUnit::Millimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_to_centimeter() {
        let res = Kilometer::new(1.0e-5).convert_scalar(&DistanceUnit::Centimeter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_to_meter() {
        let res = Kilometer::new(1.0e-3).convert_scalar(&DistanceUnit::Meter);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_to_kilometer() {
        let res = Kilometer::new(1.0).convert_scalar(&DistanceUnit::Kilometer);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }

    #[test]
    fn test_kilometers_miles() {
        let res = Kilometer::new(1.609_344).convert_scalar(&DistanceUnit::Mile);

        assert!(res.is_ok());
        assert!(is_close(1.0, res.unwrap()));
    }
}
