use crate::material::BoltMaterial;
use crate::unit::LengthUnit::*;
use crate::value::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct BoltConnection {
    num_row: u32,
    bolt: HighTensionBolt,
}

impl Default for BoltConnection {
    fn default() -> Self {
        Self {
            num_row: 1,
            bolt: HighTensionBolt::default(),
        }
    }
}

#[derive(Default)]
struct HighTensionBolt {
    diameter: BoltDiameter,
    material: BoltMaterial,
}

#[derive(EnumIter, Default)]
pub enum BoltDiameter {
    #[default]
    M20,
    M22,
}

impl BoltDiameter {
    pub fn name(&self) -> &str {
        match self {
            Self::M20 => "M20",
            Self::M22 => "M22",
        }
    }

    fn diameter_mm(&self) -> f64 {
        match self {
            Self::M20 => 20.0,
            Self::M22 => 22.0,
        }
    }

    fn diameter(&self) -> Length {
        Length::new(self.diameter_mm(), &MilliMeter)
    }
}
