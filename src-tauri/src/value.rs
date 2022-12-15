use crate::unit::*;
use std::ops;

#[derive(Debug, PartialEq)]
pub struct Length {
    m: f64,
}

impl Length {
    const DIM: i32 = 1;

    pub fn new(l: f64, unit: &LengthUnit) -> Self {
        Self {
            m: l * unit.rate().powi(Self::DIM),
        }
    }

    pub fn get_value_in(&self, unit: &LengthUnit) -> f64 {
        self.m / unit.rate().powi(Self::DIM)
    }
}

pub struct Area {
    m2: f64,
}

impl Area {
    const DIM: i32 = 2;

    pub fn new(a: f64, unit: &LengthUnit) -> Area {
        Area {
            m2: a * unit.rate().powi(Self::DIM),
        }
    }

    pub fn get_value_in(&self, unit: &LengthUnit) -> f64 {
        self.m2 / unit.rate().powi(Self::DIM)
    }
}

#[derive(Default, Copy, Clone)]
pub struct Force {
    n: f64,
}

impl Force {
    pub fn new(f: f64, unit: &ForceUnit) -> Self {
        Force { n: f * unit.rate() }
    }

    pub fn get_value_in(&self, unit: &ForceUnit) -> f64 {
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
        let s = self.n / rhs.m2;
        Stress::new(s, &ForceUnit::default(), &LengthUnit::default())
    }
}

pub struct Stress {
    n_per_m2: f64,
}

impl Stress {
    pub fn new(s: f64, fu: &ForceUnit, lu: &LengthUnit) -> Stress {
        Stress {
            n_per_m2: s * fu.rate() / lu.rate().powi(2),
        }
    }

    pub fn get_value_in(&self, fu: &ForceUnit, lu: &LengthUnit) -> f64 {
        self.n_per_m2 / fu.rate() * lu.rate().powi(2)
    }
}

impl ops::Div<Self> for Stress {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.n_per_m2 / rhs.n_per_m2
    }
}
