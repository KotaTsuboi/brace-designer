use std::sync::Mutex;

use serde::Serialize;
use tauri::{Builder, Wry};

#[derive(Serialize, Default, Clone)]
pub struct BaseYieldResult {
    pub name: String,
    pub section_name: String,
    pub material_name: String,
    pub a: f64,
    pub ae: f64,
    pub fy: f64,
    pub ny: f64,
    pub nd: f64,
    pub gamma: f64,
    pub judge: String,
}

impl BaseYieldResult {
    pub fn to_latex_table_row(&self) -> String {
        format!(
            r#"\midrule {} & {} & {} & {:.1} & {:.1} & {:.0} & {:.1} & {:.1} & {:.3} & {} \\"#,
            self.name,
            self.section_name,
            self.material_name,
            self.a,
            self.ae,
            self.fy,
            self.ny,
            self.nd,
            self.gamma,
            self.judge,
        )
    }
}

pub struct BoltYieldResult {}

pub fn manage(builder: Builder<Wry>) -> Builder<Wry> {
    builder.manage(Mutex::new(BaseYieldResult::default()))
}
