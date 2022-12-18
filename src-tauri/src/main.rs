#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod bolt;
mod material;
mod section;
mod unit;
mod value;

use crate::bolt::*;
use crate::material::*;
use crate::section::*;
use crate::unit::ForceUnit::*;
use crate::unit::LengthUnit::*;
use crate::value::Force;
use std::sync::Mutex;
use strum::IntoEnumIterator;

struct Brace {
    section: Mutex<Box<dyn Section>>,
    material: Mutex<SteelMaterial>,
    bolt_connection: Mutex<BoltConnection>,
}

impl Default for Brace {
    fn default() -> Self {
        Brace {
            section: Mutex::new(Box::new(CTSteel::default())),
            material: Mutex::new(SteelMaterial::default()),
            bolt_connection: Mutex::new(BoltConnection::default()),
        }
    }
}

#[derive(Default)]
struct AxialForce {
    force: Mutex<Force>,
}

#[tauri::command]
fn list_sections() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for section in CTSteel::iter() {
        list.push(section.name());
    }

    for section in AngleSteel::iter() {
        list.push(section.name());
    }

    for section in ChannelSteel::iter() {
        list.push(section.name());
    }

    list
}

#[tauri::command]
fn list_materials() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for material in SteelMaterial::iter() {
        list.push(material.name().to_string());
    }

    list
}

#[tauri::command]
fn list_bolt_diameters() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for diameter in BoltDiameter::iter() {
        list.push(diameter.name().to_string());
    }

    list
}

#[tauri::command]
fn list_bolt_materials() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for material in BoltMaterial::iter() {
        list.push(material.name().to_string());
    }

    list
}

#[tauri::command]
fn set_section(name: &str, brace: tauri::State<Brace>) {
    let new_section = get_section(name).unwrap();
    let mut sec = brace.section.lock().unwrap();
    *sec = new_section;
}

#[tauri::command]
fn set_material(name: &str, brace: tauri::State<Brace>) {
    let new_material = SteelMaterial::new(name).unwrap();
    let mut mat = brace.material.lock().unwrap();
    *mat = new_material;
}

#[tauri::command]
fn set_bolts(material_name: &str, diameter_name: &str, num_bolts: u32, brace: tauri::State<Brace>) {
    let new_bolts = BoltConnection::new(material_name, diameter_name, num_bolts).unwrap();
    let mut bolts = brace.bolt_connection.lock().unwrap();
    *bolts = new_bolts;
}

#[tauri::command]
fn set_force_in_kn(value: f64, n: tauri::State<AxialForce>) {
    let mut load = n.force.lock().unwrap();
    *load = Force::new(value, KiloNewton);
}

#[tauri::command]
fn get_section_in_mm(brace: tauri::State<Brace>) -> Polyline {
    brace.section.lock().unwrap().shape_in_mm()
}

#[tauri::command]
fn get_section_thickness_in_mm(brace: tauri::State<Brace>) -> f64 {
    brace
        .section
        .lock()
        .unwrap()
        .thickness()
        .get_value_in(MilliMeter)
}

#[tauri::command]
fn get_bolt_diameter_in_mm(brace: tauri::State<Brace>) -> f64 {
    brace
        .bolt_connection
        .lock()
        .unwrap()
        .bolt
        .diameter()
        .get_value_in(MilliMeter)
}

#[tauri::command]
fn get_bolt_coord_list_in_mm(brace: tauri::State<Brace>) -> Vec<(f64, f64)> {
    let gauge_list: Vec<f64> = brace
        .section
        .lock()
        .unwrap()
        .gauge_list()
        .iter()
        .map(|g| g.get_value_in(MilliMeter))
        .collect();

    let num_row = brace.bolt_connection.lock().unwrap().num_row;

    let e = 40.0;
    let p = 60.0;
    let mut list: Vec<(f64, f64)> = vec![];

    for y in gauge_list {
        for i in 0..num_row {
            let x = e + (i as f64) * p;
            list.push((x, y));
        }
    }

    list
}

#[tauri::command]
fn get_joint_length_in_mm(brace: tauri::State<Brace>) -> f64 {
    let num_row = brace.bolt_connection.lock().unwrap().num_row;

    let e = 40.0;
    let p = 60.0;

    2.0 * e + (num_row - 1) as f64 * p
}

#[tauri::command]
fn get_bolt_dimension_in_mm(brace: tauri::State<Brace>) -> (f64, f64) {
    let bolts = brace.bolt_connection.lock().unwrap();
    (
        bolts.bolt.head_height().get_value_in(MilliMeter),
        bolts.bolt.head_size().get_value_in(MilliMeter),
    )
}

#[tauri::command]
fn calculate_base(brace: tauri::State<Brace>, force: tauri::State<AxialForce>) -> f64 {
    let sec = brace.section.lock().unwrap();
    let mat = brace.material.lock().unwrap();
    let hole_diameter = (*brace.bolt_connection.lock().unwrap())
        .bolt
        .hole_diameter();
    let f = force.force.lock().unwrap();
    let hole_area = (*sec).thickness() * hole_diameter * (*sec).num_bolt_col() as f64;
    let effective_area = (*sec).area() - hole_area;
    *f / effective_area / (*mat).get_fy()
}

#[tauri::command]
fn calculate_bolts(brace: tauri::State<Brace>, force: tauri::State<AxialForce>) -> f64 {
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
    tauri::Builder::default()
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
            set_force_in_kn,
            get_section_in_mm,
            get_section_thickness_in_mm,
            get_bolt_diameter_in_mm,
            get_bolt_coord_list_in_mm,
            get_joint_length_in_mm,
            get_bolt_dimension_in_mm,
            calculate_base,
            calculate_bolts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
