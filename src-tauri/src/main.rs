#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod material;
mod section;
mod unit;
mod value;

use crate::section::*;
use crate::unit::*;
use std::sync::Mutex;
use strum::IntoEnumIterator;

#[derive(Default)]
struct Brace<T>
where
    T: Section + Default,
{
    section: Mutex<T>,
}

#[tauri::command]
fn list_sections() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for section in AngleSteel::iter() {
        list.push(section.name());
    }

    for section in ChannelSteel::iter() {
        list.push(section.name());
    }

    list
}

#[tauri::command]
fn is_angle_section(name: String) -> bool {
    AngleSteel::new(name).is_some()
}

#[tauri::command]
fn set_angle_section(name: String, brace: tauri::State<Brace<AngleSteel>>) {
    let angle = AngleSteel::new(name).unwrap();
    let mut section = brace.section.lock().unwrap();
    *section = angle;
}

#[tauri::command]
fn get_angle_a_as_mm(brace: tauri::State<Brace<AngleSteel>>) -> f64 {
    brace
        .section
        .lock()
        .unwrap()
        .a()
        .get_value_in(LengthUnit::MilliMeter)
}

#[tauri::command]
fn get_angle_b_as_mm(brace: tauri::State<Brace<AngleSteel>>) -> f64 {
    brace
        .section
        .lock()
        .unwrap()
        .b()
        .get_value_in(LengthUnit::MilliMeter)
}

#[tauri::command]
fn get_angle_t_as_mm(brace: tauri::State<Brace<AngleSteel>>) -> f64 {
    brace
        .section
        .lock()
        .unwrap()
        .t()
        .get_value_in(LengthUnit::MilliMeter)
}

#[tauri::command]
fn is_channel_section(name: String) -> bool {
    ChannelSteel::new(name).is_some()
}

#[tauri::command]
fn set_channel_section(name: String, brace: tauri::State<Brace<ChannelSteel>>) {
    let channel = ChannelSteel::new(name).unwrap();
    let mut section = brace.section.lock().unwrap();
    *section = channel;
}

fn main() {
    tauri::Builder::default()
        .manage(Brace::<AngleSteel>::default())
        .invoke_handler(tauri::generate_handler![
            list_sections,
            is_angle_section,
            set_angle_section,
            get_angle_a_as_mm,
            get_angle_b_as_mm,
            get_angle_t_as_mm,
            is_channel_section,
            set_channel_section,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
