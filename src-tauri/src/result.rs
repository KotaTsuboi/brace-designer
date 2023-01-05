use std::sync::Mutex;

use serde::Serialize;
use strum_macros::Display;
use tauri::{Builder, Wry};

use crate::unit::ForceUnit::*;
use crate::unit::LengthUnit::*;
use crate::value::{area::Area, force::Force, stress::Stress};

#[derive(Display, Default, Serialize, Clone)]
pub enum Judge {
    OK,
    #[default]
    NG,
}

#[derive(Serialize, Default, Clone)]
pub struct BaseYieldResult {
    pub name: String,
    pub section_name: String,
    pub material_name: String,
    pub a: Area,
    pub ae: Area,
    pub fy: Stress,
    pub ny: Force,
    pub nd: Force,
    pub gamma: f64,
    pub judge: Judge,
}

impl BaseYieldResult {
    pub fn to_latex_table_row(&self) -> String {
        format!(
            r#"\midrule {} & {} & {} & {:.1} & {:.1} & {:.0} & {:.1} & {:.1} & {:.3} & {} \\"#,
            self.name,
            self.section_name,
            self.material_name,
            self.a.get_value_in(CentiMeter),
            self.ae.get_value_in(CentiMeter),
            self.fy.get_value_in(Newton, MilliMeter),
            self.ny.get_value_in(KiloNewton),
            self.nd.get_value_in(KiloNewton),
            self.gamma,
            self.judge,
        )
    }
}

pub struct BoltYieldResult {}

pub fn manage(builder: Builder<Wry>) -> Builder<Wry> {
    builder.manage(Mutex::new(BaseYieldResult::default()))
}
