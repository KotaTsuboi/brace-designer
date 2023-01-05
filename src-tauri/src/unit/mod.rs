use serde::Serialize;

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize)]
pub enum LengthUnit {
    #[default]
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

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize)]
pub enum ForceUnit {
    #[default]
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
