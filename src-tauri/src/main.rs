#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod model;
mod result;
mod sheet;
mod unit;
mod value;

use crate::model::bolt::command::*;
use crate::model::bolt::*;
use crate::model::gpl::*;
use crate::model::material;
use crate::model::material::command::*;
use crate::model::material::*;
use crate::model::section;
use crate::model::section::command::*;
use crate::model::section::Polyline;
use crate::model::AxialForce;
use crate::model::Brace;
use crate::result::BaseYieldResult;
use crate::sheet::command::*;
use crate::unit::ForceUnit::*;
use crate::unit::LengthUnit;
use crate::unit::LengthUnit::*;
use crate::value::area::Area;
use crate::value::force::Force;
use crate::value::length::Length;
use result::Judge;
use std::cmp;
use std::f64::consts::PI;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
fn set_section(name: &str, brace: State<Brace>) {
    let new_section = section::get_section(name).unwrap();
    let mut sec = brace.section.lock().unwrap();
    *sec = new_section;
}

#[tauri::command]
fn set_material(name: &str, brace: State<Brace>) {
    let new_material = SteelMaterial::new(name).unwrap();
    let mut mat = brace.material.lock().unwrap();
    *mat = new_material;
}

#[tauri::command]
fn set_bolts(material_name: &str, diameter_name: &str, num_bolts: u32, brace: State<Brace>) {
    let new_bolts = BoltConnection::new(material_name, diameter_name, num_bolts).unwrap();
    let mut bolts = brace.bolt_connection.lock().unwrap();
    *bolts = new_bolts;
}

#[tauri::command]
fn set_gpl(thickness: f64, lg: f64, material_name: &str, brace: State<Brace>) {
    let new_gpl = GussetPlate::new(
        Length::new(thickness, MilliMeter),
        Length::new(lg, MilliMeter),
        SteelMaterial::new(material_name).unwrap(),
    );
    let mut gpl = brace.gpl.lock().unwrap();
    *gpl = new_gpl;
}

#[tauri::command]
fn set_force_in_kn(value: f64, n: State<AxialForce>) {
    let mut load = n.force.lock().unwrap();
    *load = Force::new(value, KiloNewton);
}

#[tauri::command]
fn get_section_in_m(brace: State<Brace>) -> Polyline {
    brace.section.lock().unwrap().shape_in_m()
}

#[tauri::command]
fn get_section_thickness_in_m(brace: State<Brace>) -> f64 {
    brace
        .section
        .lock()
        .unwrap()
        .thickness()
        .get_value_in(Meter)
}

#[tauri::command]
fn get_gpl_thickness_in_m(brace: State<Brace>) -> f64 {
    let gpl = brace.gpl.lock().unwrap();
    gpl.thickness.get_value_in(Meter)
}

#[tauri::command]
fn get_gpl_shape_in_m(brace: State<Brace>) -> Polyline {
    let gpl = brace.gpl.lock().unwrap();
    let section = brace.section.lock().unwrap();
    let bolt_connection = brace.bolt_connection.lock().unwrap();

    let unit = Meter;
    let lg = gpl.lg.get_value_in(unit);
    let breadth = section.breadth().get_value_in(unit);
    let joint_length = bolt_connection.joint_length().get_value_in(unit);
    let margin = Length::new(40.0, MilliMeter).get_value_in(unit);

    Polyline::new(vec![
        (0.0, breadth / 2.0),
        (joint_length, lg / 2.0),
        (joint_length + margin, lg / 2.0),
        (joint_length + margin, -lg / 2.0),
        (joint_length, -lg / 2.0),
        (0.0, -breadth / 2.0),
    ])
}

#[tauri::command]
fn get_bolt_diameter_in_m(brace: State<Brace>) -> f64 {
    brace
        .bolt_connection
        .lock()
        .unwrap()
        .bolt
        .diameter()
        .get_value_in(Meter)
}

#[tauri::command]
fn get_bolt_coord_list_in_m(brace: State<Brace>) -> Vec<(f64, f64)> {
    let gauge_list: Vec<Length> = brace.section.lock().unwrap().gauge_list();

    let bolt_connection = brace.bolt_connection.lock().unwrap();
    let num_row = bolt_connection.num_row;

    let e = bolt_connection.end_distance();
    let p = bolt_connection.pitch();
    let mut list: Vec<(f64, f64)> = vec![];

    for y in gauge_list {
        for i in 0..num_row {
            let x = e + p * (i as i32);
            list.push((x.get_value_in(Meter), y.get_value_in(Meter)));
        }
    }

    list
}

#[tauri::command]
fn get_joint_length_in_m(brace: State<Brace>) -> f64 {
    let bolt_connection = brace.bolt_connection.lock().unwrap();
    let n = bolt_connection.num_row as i32;
    let e = bolt_connection.end_distance();
    let p = bolt_connection.pitch();

    (e * 2 + p * (n - 1)).get_value_in(Meter)
}

#[tauri::command]
fn get_bolt_dimension_in_m(brace: State<Brace>) -> (f64, f64) {
    let bolts = brace.bolt_connection.lock().unwrap();
    (
        bolts.bolt.head_height().get_value_in(Meter),
        bolts.bolt.head_size().get_value_in(Meter),
    )
}

#[tauri::command]
fn calculate_base(
    brace: State<Brace>,
    force: State<AxialForce>,
    result: State<Mutex<BaseYieldResult>>,
) -> BaseYieldResult {
    let sec = brace.section.lock().unwrap();
    let mat = brace.material.lock().unwrap();
    let hole_diameter = (*brace.bolt_connection.lock().unwrap())
        .bolt
        .hole_diameter();
    let nd = force.force.lock().unwrap();
    let hole_area = (*sec).thickness() * hole_diameter * (*sec).num_bolt_col() as f64;
    let effective_area = (*sec).area() - hole_area;
    let fy = mat.get_fy();
    let ny = effective_area * fy;
    let gamma = (*nd) / ny;
    let judge = if gamma > 1.0 { Judge::NG } else { Judge::OK };

    let new_result = BaseYieldResult {
        name: "V1".to_string(),
        section_name: sec.name(),
        material_name: mat.name().to_string(),
        a: sec.area(),
        ae: effective_area,
        fy: mat.get_fy(),
        ny,
        nd: *nd,
        gamma,
        judge,
    };

    let mut result = result.lock().unwrap();
    *result = new_result.clone();

    new_result
}

#[tauri::command]
fn calculate_gpl(brace: State<Brace>, force: State<AxialForce>) -> f64 {
    let sec = brace.section.lock().unwrap();
    let gpl = brace.gpl.lock().unwrap();
    let bolt_connection = brace.bolt_connection.lock().unwrap();
    let hole_diameter = bolt_connection.bolt.hole_diameter();
    let f = force.force.lock().unwrap();
    let hole_area = gpl.thickness * hole_diameter * sec.num_bolt_col() as f64;
    let length =
        bolt_connection.pitch() * (bolt_connection.num_row as i32 - 1) as f64 * (PI / 6.0).tan()
            + sec.gauge_width();
    let effective_length = cmp::min(gpl.lg, length);
    let area = effective_length * gpl.thickness;
    let effective_area = if area > hole_area {
        area - hole_area
    } else {
        Area::new(0.0, LengthUnit::default())
    };
    *f / effective_area / gpl.material.get_fy()
}

#[tauri::command]
fn calculate_bolts(brace: State<Brace>, force: State<AxialForce>) -> f64 {
    let num_row = brace.bolt_connection.lock().unwrap().num_row;
    let num_col = brace.section.lock().unwrap().gauge_list().len();
    let num_bolts = num_row * num_col as u32;
    let fs = brace
        .bolt_connection
        .lock()
        .unwrap()
        .bolt
        .allowable_shear_short_single_friction();
    let f = force.force.lock().unwrap();
    *f / (fs * num_bolts as f64)
}

fn main() {
    let builder = tauri::Builder::default()
        .manage(Brace::default())
        .manage(AxialForce::default())
        .invoke_handler(tauri::generate_handler![
            list_sections,
            list_materials,
            list_bolt_diameters,
            list_bolt_materials,
            set_section,
            set_material,
            set_bolts,
            set_gpl,
            set_force_in_kn,
            get_section_in_m,
            get_section_thickness_in_m,
            get_bolt_diameter_in_m,
            get_bolt_coord_list_in_m,
            get_joint_length_in_m,
            get_bolt_dimension_in_m,
            get_gpl_thickness_in_m,
            get_gpl_shape_in_m,
            calculate_base,
            calculate_bolts,
            calculate_gpl,
            write_file,
        ]);

    let builder = result::manage(builder);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
