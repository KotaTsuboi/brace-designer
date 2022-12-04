#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod material;
mod section;
mod unit;
mod value;

use crate::material::*;
use crate::section::*;
use crate::unit::ForceUnit::*;
use crate::value::Force;
use std::sync::Mutex;
use strum::IntoEnumIterator;

struct Brace {
    section: Mutex<Box<dyn Section>>,
    material: Mutex<Box<dyn Material>>,
}

impl Default for Brace {
    fn default() -> Self {
        Brace {
            section: Mutex::new(Box::new(AngleSteel::default())),
            material: Mutex::new(Box::new(SteelMaterial::default())),
        }
    }
}

#[derive(Default)]
struct AxialForce {
    force: Mutex<Force>,
}

#[tauri::command]
fn calculate(brace: tauri::State<Brace>, force: tauri::State<AxialForce>) -> f64 {
    let sec = brace.section.lock().unwrap();
    let mat = brace.material.lock().unwrap();
    let f = force.force.lock().unwrap();
    *f / (*sec).area() / (*mat).get_fy()
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
fn set_section(name: &str, brace: tauri::State<Brace>) {
    let new_section = get_section(name).unwrap();
    let mut sec = brace.section.lock().unwrap();
    *sec = new_section;
}

#[tauri::command]
fn set_material(name: &str, brace: tauri::State<Brace>) {
    let new_material = get_material(name).unwrap();
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

fn main() {
    tauri::Builder::default()
        .manage(Brace::default())
        .manage(AxialForce::default())
        .invoke_handler(tauri::generate_handler![
            list_sections,
            set_section,
            list_materials,
            get_section_in_mm,
            set_material,
            calculate,
            set_force_in_kn
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
