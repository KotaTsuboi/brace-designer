pub mod bolt;
pub mod gpl;
pub mod material;
pub mod section;

use self::bolt::BoltConnection;
use self::material::SteelMaterial;
use self::section::Section;
use crate::model::gpl::*;
use crate::model::section::ct::CTSteel;
use crate::unit::LengthUnit::*;
use crate::value::force::Force;
use crate::value::length::Length;
use std::sync::Mutex;

pub struct Brace {
    pub section: Mutex<Box<dyn Section>>,
    pub material: Mutex<SteelMaterial>,
    pub bolt_connection: Mutex<BoltConnection>,
    pub gpl: Mutex<GussetPlate>,
}

impl Default for Brace {
    fn default() -> Self {
        Brace {
            section: Mutex::new(Box::new(CTSteel::default())),
            material: Mutex::new(SteelMaterial::default()),
            bolt_connection: Mutex::new(BoltConnection::default()),
            gpl: Mutex::new(GussetPlate::new(
                CTSteel::default().thickness(),
                Length::new(300.0, MilliMeter),
                SteelMaterial::SS400,
            )),
        }
    }
}

#[derive(Default)]
pub struct AxialForce {
    pub force: Mutex<Force>,
}
