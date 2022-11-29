use crate::unit::ForceUnit::*;
use crate::unit::LengthUnit::*;
use crate::value::Stress;

pub trait Material {
    fn get_fy(&self) -> Stress;
    fn get_fu(&self) -> Stress;
}

pub enum SteelMaterial {
    SS400,
    SM490,
}

impl Material for SteelMaterial {
    fn get_fy(&self) -> Stress {
        match self {
            Self::SS400 => Stress::new(235.0, Newton, MilliMeter),
            Self::SM490 => Stress::new(325.0, Newton, MilliMeter),
        }
    }

    fn get_fu(&self) -> Stress {
        match self {
            Self::SS400 => Stress::new(400.0, Newton, MilliMeter),
            Self::SM490 => Stress::new(490.0, Newton, MilliMeter),
        }
    }
}
