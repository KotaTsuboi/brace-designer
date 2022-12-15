use crate::material::BoltMaterial;
use crate::unit::LengthUnit::*;
use crate::value::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct BoltConnection {
    pub bolt: HighTensionBolt,
    pub num_row: u32,
}

impl BoltConnection {
    pub fn new(material_name: &str, diameter_name: &str, num_row: u32) -> Option<Self> {
        if let Some(bolt) = HighTensionBolt::new(material_name, diameter_name) {
            return Some(Self { bolt, num_row });
        }
        None
    }

    /*
    pub fn diameter(&self) -> Length {
        self.bolt.diameter()
    }

    pub fn head_height(&self) -> Length {
        self.bolt.head_height()
    }

    pub fn head_size(&self) -> Length {
        self.bolt.head_size()
    }
    */
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
pub struct HighTensionBolt {
    diameter: BoltDiameter,
    material: BoltMaterial,
}

impl HighTensionBolt {
    fn new(material_name: &str, diameter_name: &str) -> Option<Self> {
        if let Some(m) = BoltMaterial::new(material_name) {
            if let Some(d) = BoltDiameter::new(diameter_name) {
                return Some(Self {
                    material: m,
                    diameter: d,
                });
            }
        }
        None
    }

    pub fn diameter(&self) -> Length {
        self.diameter.d()
    }

    pub fn head_height(&self) -> Length {
        self.diameter.h()
    }

    pub fn head_size(&self) -> Length {
        self.diameter.large_d()
    }

    pub fn allowable_shear_short_single_friction(&self) -> Force {
        self.diameter.full_area() * self.material.get_t0() * 0.45
    }
}

#[derive(EnumIter, Default)]
pub enum BoltDiameter {
    #[default]
    M20,
    M22,
}

impl BoltDiameter {
    fn new(name: &str) -> Option<Self> {
        Self::iter().find(|d| d.name() == name)
    }

    pub fn name(&self) -> &str {
        match self {
            Self::M20 => "M20",
            Self::M22 => "M22",
        }
    }

    fn d_mm(&self) -> f64 {
        match self {
            Self::M20 => 20.0,
            Self::M22 => 22.0,
        }
    }

    fn d(&self) -> Length {
        Length::new(self.d_mm(), &MilliMeter)
    }

    fn h_mm(&self) -> f64 {
        match self {
            Self::M20 => 13.0,
            Self::M22 => 14.0,
        }
    }

    fn h(&self) -> Length {
        Length::new(self.h_mm(), &MilliMeter)
    }

    fn large_d_mm(&self) -> f64 {
        match self {
            Self::M20 => 30.0,
            Self::M22 => 34.0,
        }
    }

    fn large_d(&self) -> Length {
        Length::new(self.large_d_mm(), &MilliMeter)
    }

    fn full_area_mm(&self) -> f64 {
        match self {
            Self::M20 => 314.0,
            Self::M22 => 380.0,
        }
    }

    fn full_area(&self) -> Area {
        Area::new(self.full_area_mm(), &MilliMeter)
    }
}
