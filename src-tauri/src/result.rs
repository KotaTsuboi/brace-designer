use std::sync::Mutex;

use serde::Serialize;
use strum_macros::Display;
use tauri::{Builder, Wry};

use crate::unit::ForceUnit::*;
use crate::unit::LengthUnit::*;
use crate::value::length::Length;
use crate::value::{area::Area, force::Force, stress::Stress};

#[derive(Display, Default, Serialize, Clone)]
pub enum Judge {
    OK,
    #[default]
    NG,
}

#[derive(Default, Clone)]
pub struct BraceResult {
    pub base_yield_result: BaseYieldResult,
    pub bolt_yield_result: BoltYieldResult,
    pub gpl_yield_result: GplYieldResult,
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

#[derive(Serialize, Default, Clone)]
pub struct BoltYieldResult {
    pub name: String,
    pub diameter_name: String,
    pub material_name: String,
    pub qy: Force,
    pub num_bolts: u32,
    pub ny: Force,
    pub nd: Force,
    pub gamma: f64,
    pub judge: Judge,
}

impl BoltYieldResult {
    pub fn to_latex_table_row(&self) -> String {
        format!(
            r#"\midrule {} & {} & {} & {} & {} & {} & {} & {:.3} & {} \\"#,
            self.name,
            self.diameter_name,
            self.material_name,
            self.qy.get_value_in(KiloNewton),
            self.num_bolts,
            self.ny.get_value_in(KiloNewton),
            self.nd.get_value_in(KiloNewton),
            self.gamma,
            self.judge
        )
    }
}

#[derive(Serialize, Default, Clone)]
pub struct GplYieldResult {
    pub name: String,
    pub material_name: String,
    pub lg: Length,
    pub thickness: Length,
    pub a: Area,
    pub ae: Area,
    pub fy: Stress,
    pub ny: Force,
    pub nd: Force,
    pub gamma: f64,
    pub judge: Judge,
}

impl GplYieldResult {
    pub fn to_latex_table_row(&self) -> String {
        format!(
            r#"\midrule {} & {} & {:.0} & {:.0} & {:.1} & {:.1} & {:.0} & {:.1} & {:.1} & {:.3} & {} \\"#,
            self.name,
            self.material_name,
            self.lg.get_value_in(MilliMeter),
            self.thickness.get_value_in(MilliMeter),
            self.a.get_value_in(CentiMeter),
            self.ae.get_value_in(CentiMeter),
            self.fy.get_value_in(Newton, MilliMeter),
            self.ny.get_value_in(KiloNewton),
            self.nd.get_value_in(KiloNewton),
            self.gamma,
            self.judge
        )
    }
}

pub fn manage(builder: Builder<Wry>) -> Builder<Wry> {
    builder.manage(Mutex::new(BraceResult::default()))
}
