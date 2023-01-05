use serde::Serialize;

use crate::unit::*;
use std::cmp::Ordering;
use std::ops;

use super::force::Force;
use super::stress::Stress;

#[derive(Copy, Clone, Default, Serialize)]
pub struct Area {
    m2: f64,
}

impl Area {
    const DIM: i32 = 2;

    pub fn new(a: f64, unit: LengthUnit) -> Area {
        if a < 0.0 {
            panic!("Area is negative");
        }

        Area {
            m2: a * unit.rate().powi(Self::DIM),
        }
    }

    pub fn get_value_in(&self, unit: LengthUnit) -> f64 {
        self.m2 / unit.rate().powi(Self::DIM)
    }
}

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        let unit = LengthUnit::default();
        self.get_value_in(unit) == other.get_value_in(unit)
    }
}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let unit = LengthUnit::default();
        self.get_value_in(unit)
            .partial_cmp(&other.get_value_in(unit))
    }
}

impl ops::Mul<f64> for Area {
    type Output = Area;

    fn mul(self, rhs: f64) -> Self::Output {
        Area::new(self.m2 * rhs, LengthUnit::default())
    }
}

impl ops::Add<Area> for Area {
    type Output = Area;

    fn add(self, rhs: Area) -> Self::Output {
        Area::new(self.m2 + rhs.m2, LengthUnit::default())
    }
}

impl ops::Sub<Area> for Area {
    type Output = Area;

    fn sub(self, rhs: Area) -> Self::Output {
        Area::new(self.m2 - rhs.m2, LengthUnit::default())
    }
}

impl ops::Mul<Stress> for Area {
    type Output = Force;

    fn mul(self, rhs: Stress) -> Self::Output {
        Force::new(
            self.m2 * rhs.get_value_in(ForceUnit::default(), LengthUnit::default()),
            ForceUnit::default(),
        )
    }
}
