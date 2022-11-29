use crate::unit::*;

pub struct Length {
    m: f64,
}

impl Length {
    const DIM: i32 = 1;

    pub fn new(l: f64, unit: LengthUnit) -> Self {
        Self {
            m: l * unit.rate().powi(Self::DIM),
        }
    }

    pub fn get_value_in(&self, unit: LengthUnit) -> f64 {
        self.m / unit.rate().powi(Self::DIM)
    }
}

pub struct Area {
    m2: f64,
}

impl Area {
    const DIM: i32 = 2;

    pub fn new(a: f64, unit: LengthUnit) -> Area {
        Area {
            m2: a * unit.rate().powi(Self::DIM),
        }
    }

    pub fn get_value_in(&self, unit: LengthUnit) -> f64 {
        self.m2 / unit.rate().powi(Self::DIM)
    }
}

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
