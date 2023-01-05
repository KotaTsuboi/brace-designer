use serde::Serialize;

use crate::unit::*;
use std::ops;

#[derive(Copy, Clone, Default, Serialize)]
pub struct Stress {
    n_per_m2: f64,
}

impl Stress {
    pub fn new(s: f64, fu: ForceUnit, lu: LengthUnit) -> Stress {
        Stress {
            n_per_m2: s * fu.rate() / lu.rate().powi(2),
        }
    }

    pub fn get_value_in(&self, fu: ForceUnit, lu: LengthUnit) -> f64 {
        self.n_per_m2 / fu.rate() * lu.rate().powi(2)
    }
}

impl ops::Div<Self> for Stress {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.n_per_m2 / rhs.n_per_m2
    }
}
