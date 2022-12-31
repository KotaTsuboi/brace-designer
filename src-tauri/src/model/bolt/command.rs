use super::*;
use strum::IntoEnumIterator;

#[tauri::command]
pub fn list_bolt_diameters() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for diameter in BoltDiameter::iter() {
        list.push(diameter.name().to_string());
    }

    list
}
