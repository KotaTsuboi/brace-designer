use serde::Serialize;

use crate::unit::*;
use std::cmp::Ordering;
use std::ops;

use super::area::Area;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Length {
    value: f64,
    unit: LengthUnit,
}

impl Length {
    pub const fn new(value: f64, unit: LengthUnit) -> Self {
        Self { value, unit }
    }

    pub fn get_value_in(&self, unit: LengthUnit) -> f64 {
        self.value * self.unit.rate() / unit.rate()
    }
}

impl ops::Add<Length> for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Self::Output {
        Length::new(
            self.get_value_in(self.unit) + rhs.get_value_in(self.unit),
            self.unit,
        )
    }
}

impl ops::Sub<Length> for Length {
    type Output = Length;

    fn sub(self, rhs: Length) -> Self::Output {
        Length::new(
            self.get_value_in(self.unit) - rhs.get_value_in(self.unit),
            self.unit,
        )
    }
}

impl ops::Mul<i32> for Length {
    type Output = Length;

    fn mul(self, rhs: i32) -> Self::Output {
        Length::new(self.get_value_in(self.unit) * rhs as f64, self.unit)
    }
}

impl ops::Mul<f64> for Length {
    type Output = Length;

    fn mul(self, rhs: f64) -> Self::Output {
        Length::new(self.get_value_in(self.unit) * rhs, self.unit)
    }
}

impl ops::Mul<Length> for Length {
    type Output = Area;

    fn mul(self, rhs: Length) -> Self::Output {
        let unit = LengthUnit::default();
        Area::new(self.get_value_in(unit) * rhs.get_value_in(unit), unit)
    }
}

impl PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        let unit = LengthUnit::default();
        self.get_value_in(unit) == other.get_value_in(unit)
    }
}

impl Eq for Length {}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let unit = LengthUnit::default();
        self.get_value_in(unit)
            .partial_cmp(&other.get_value_in(unit))
    }
}

impl Ord for Length {
    fn cmp(&self, other: &Self) -> Ordering {
        let unit = LengthUnit::default();
        self.get_value_in(unit)
            .partial_cmp(&other.get_value_in(unit))
            .unwrap()
    }
}
