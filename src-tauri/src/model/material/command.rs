use super::*;
use strum::IntoEnumIterator;

#[tauri::command]
pub fn list_materials() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for material in SteelMaterial::iter() {
        list.push(material.name().to_string());
    }

    list
}

#[tauri::command]
pub fn list_bolt_materials() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for material in BoltMaterial::iter() {
        list.push(material.name().to_string());
    }

    list
}
