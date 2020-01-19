use crate::traits::*;
use std::fmt;

const JULIAN_DAY_LEAP: f64 = 1_720_994.5; // additional 0.5 means that day starts at midnight

pub fn is_leap_year(year: u16) -> bool {
    match year.rem_euclid(4) {
        0 if year.rem_euclid(100) == 0 && year.rem_euclid(400) != 0 => false,
        0 => true,
        _ => false,
    }
}

pub struct DateTime {
    year: i16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    z: i8, // 1/24th from greenwich, kind of timezone
}

impl DateTime {
    pub fn new(year: i16, month: u8, day: u8, hour: u8, minute: u8, second: u8, z: i8) -> Self {
        assert!(month > 0 && month <= 12, "month must be in range 1..=12");
        assert!(day > 0 && day <= 31, "day must be in range 1..=31");
        assert!(hour < 24, "hour must be in range 0..=23");
        assert!(minute < 60, "minute must be in range 0..=59");
        assert!(second < 60, "second must be in range 0..=59");
        assert!(z >= -12 && z <= 12, "zone must be in range -12..=12");

        DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            z,
        }
    }

    pub fn from_date(year: i16, month: u8, day: u8) -> Self {
        DateTime::new(year, month, day, 0, 0, 0, 0)
    }

    pub fn is_gregorian(&self) -> bool {
        self.year > 1583 || (self.year == 1583 && self.month >= 10 && self.day >= 15)
    }

    pub fn fractional_day(&self) -> f64 {
        (self.day as f64)
            + (self.hour as f64) / 24.0
            + (self.minute as f64) / (24.0 * 60.0)
            + (self.second as f64) / (24.0 * 60.0 * 60.0)
    }

    pub fn to_julian_day(&self) -> f64 {
        let (y, m) = if self.month > 2 {
            (self.year, self.month)
        } else {
            (self.year - 1, self.month + 12)
        };

        let t = if self.year < 0 { 0.75 } else { 0.0 };
        let a = if self.is_gregorian() {
            ((self.year as f64) / 100.0).trunc()
        } else {
            0.0
        };

        let b = if self.is_gregorian() {
            2.0 - a + (a / 4.0).trunc()
        } else {
            0.0
        };

        b + (365.25 * (y as f64) - t).trunc()
            + (30.6001 * (m + 1) as f64).trunc()
            + self.fractional_day()
            + JULIAN_DAY_LEAP
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_test_helpers::*;

    #[test]
    fn test_leap_year() {
        assert_eq!(false, is_leap_year(1906));
        assert_eq!(true, is_leap_year(1908));
        assert_eq!(false, is_leap_year(1800));
        assert_eq!(true, is_leap_year(1600));
    }

    #[test]
    fn test_to_julian_day() {
        let dt = DateTime::from_date(2010, 1, 1);

        assert_close(2_455_197.50, dt.to_julian_day());
    }

    #[test]
    fn test_to_julian_day_with_fractional_day() {
        let dt = DateTime::new(2015, 3, 21, 12, 0, 0, 0);

        assert_close(2_457_103.0, dt.to_julian_day());
    }
}
