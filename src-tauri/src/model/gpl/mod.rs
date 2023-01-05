use crate::material::*;
use crate::value::area::Area;
use crate::value::length::Length;

pub struct GussetPlate {
    pub thickness: Length,
    pub lg: Length,
    pub material: SteelMaterial,
}

impl GussetPlate {
    pub fn new(thickness: Length, lg: Length, material: SteelMaterial) -> Self {
        Self {
            thickness,
            lg,
            material,
        }
    }

    pub fn area(&self) -> Area {
        self.thickness * self.lg
    }
}
