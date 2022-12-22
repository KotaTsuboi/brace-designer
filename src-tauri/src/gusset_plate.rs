use crate::material::*;
use crate::unit::LengthUnit::*;
use crate::value::*;

pub struct GussetPlate {
    thickness: Length,
    lg: Length,
    material: SteelMaterial,
}

impl GussetPlate {
    pub fn new(thickness: Length, lg: Length, material: SteelMaterial) -> Self {
        Self {
            thickness,
            lg,
            material,
        }
    }
}
