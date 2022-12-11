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
            section: Mutex::new(Box::new(AngleSteel::default())),
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

    for section in AngleSteel::iter() {
        list.push(section.name().to_string());
    }

    for section in ChannelSteel::iter() {
        list.push(section.name().to_string());
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
fn set_force_in_kn(value: f64, n: tauri::State<AxialForce>) {
    let mut load = n.force.lock().unwrap();
    *load = Force::new(value, &KiloNewton);
}

#[tauri::command]
fn get_section_in_mm(brace: tauri::State<Brace>) -> Polyline {
    brace.section.lock().unwrap().shape_in_mm()
}

#[tauri::command]
fn calculate(brace: tauri::State<Brace>, force: tauri::State<AxialForce>) -> f64 {
    let sec = brace.section.lock().unwrap();
    let mat = brace.material.lock().unwrap();
    let f = force.force.lock().unwrap();
    *f / (*sec).area() / (*mat).get_fy()
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
            set_force_in_kn,
            get_section_in_mm,
            calculate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
