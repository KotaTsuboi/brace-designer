use strum::IntoEnumIterator;

use super::WeldingType;

#[tauri::command]
pub fn list_welding_types() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for welding_type in WeldingType::iter() {
        list.push(welding_type.name());
    }

    list
}
