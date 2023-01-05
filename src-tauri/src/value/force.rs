use serde::Serialize;

use crate::unit::*;
use std::ops;

use super::{area::Area, stress::Stress};

#[derive(Default, Copy, Clone, Serialize)]
pub struct Force {
    n: f64,
}

impl Force {
    pub fn new(f: f64, unit: ForceUnit) -> Self {
        Force { n: f * unit.rate() }
    }

    pub fn get_value_in(&self, unit: ForceUnit) -> f64 {
        self.n / unit.rate()
    }
}

impl ops::Mul<f64> for Force {
    type Output = Force;

    fn mul(self, rhs: f64) -> Self::Output {
        Force { n: self.n * rhs }
    }
}

impl ops::Div<Area> for Force {
    type Output = Stress;

    fn div(self, rhs: Area) -> Self::Output {
        let s = self.n / rhs.get_value_in(LengthUnit::default());
        Stress::new(s, ForceUnit::default(), LengthUnit::default())
    }
}

impl ops::Div<Force> for Force {
    type Output = f64;

    fn div(self, rhs: Force) -> Self::Output {
        self.n / rhs.n
    }
}
