use crate::unit::ForceUnit::*;
use crate::unit::LengthUnit::*;
use crate::value::Stress;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Material: Send + Sync {}

#[derive(EnumIter, Default)]
pub enum SteelMaterial {
    #[default]
    SS400,
    SM490,
}

impl SteelMaterial {
    pub fn new(name: &str) -> Option<Self> {
        Self::iter().find(|material| material.name() == name)
    }

    pub fn name(&self) -> &str {
        match self {
            Self::SS400 => "SS400",
            Self::SM490 => "SM490",
        }
    }

    pub fn get_fy(&self) -> Stress {
        match self {
            Self::SS400 => Stress::new(235.0, &Newton, &MilliMeter),
            Self::SM490 => Stress::new(325.0, &Newton, &MilliMeter),
        }
    }

    pub fn get_fu(&self) -> Stress {
        match self {
            Self::SS400 => Stress::new(400.0, &Newton, &MilliMeter),
            Self::SM490 => Stress::new(490.0, &Newton, &MilliMeter),
        }
    }
}

impl Material for SteelMaterial {}

#[derive(EnumIter, Default)]
pub enum BoltMaterial {
    F8T,
    #[default]
    F10T,
}

impl BoltMaterial {
    pub fn new(name: &str) -> Option<Self> {
        Self::iter().find(|material| material.name() == name)
    }

    pub fn name(&self) -> &str {
        match self {
            Self::F8T => "F8T",
            Self::F10T => "F10T",
        }
    }

    fn get_t0_nmm(&self) -> f64 {
        match self {
            Self::F8T => 400.0,
            Self::F10T => 500.0,
        }
    }

    fn get_t0(&self) -> Stress {
        Stress::new(self.get_t0_nmm(), &Newton, &MilliMeter)
    }
}

impl Material for BoltMaterial {}
