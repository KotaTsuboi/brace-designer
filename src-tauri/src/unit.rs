pub enum LengthUnit {
    Meter,
    CentiMeter,
    MilliMeter,
}

impl LengthUnit {
    pub fn rate(&self) -> f64 {
        match self {
            Self::Meter => 1e+0,
            Self::CentiMeter => 1e-2,
            Self::MilliMeter => 1e-3,
        }
    }
}

pub enum ForceUnit {
    Newton,
    KiloNewton,
}

impl ForceUnit {
    pub fn rate(&self) -> f64 {
        match self {
            Self::Newton => 1e+0,
            Self::KiloNewton => 1e+3,
        }
    }
}
