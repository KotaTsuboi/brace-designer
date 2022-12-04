use crate::unit::ForceUnit::*;
use crate::unit::LengthUnit::*;
use crate::value::Stress;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn get_material(name: &str) -> Option<Box<dyn Material>> {
    if let Some(material) = SteelMaterial::new(name) {
        return Some(Box::new(material));
    }

    None
}

pub trait Material: Send + Sync {
    fn get_fy(&self) -> Stress;
    fn get_fu(&self) -> Stress;
}

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
}

impl Material for SteelMaterial {
    fn get_fy(&self) -> Stress {
        match self {
            Self::SS400 => Stress::new(235.0, &Newton, &MilliMeter),
            Self::SM490 => Stress::new(325.0, &Newton, &MilliMeter),
        }
    }

    fn get_fu(&self) -> Stress {
        match self {
            Self::SS400 => Stress::new(400.0, &Newton, &MilliMeter),
            Self::SM490 => Stress::new(490.0, &Newton, &MilliMeter),
        }
    }
}
