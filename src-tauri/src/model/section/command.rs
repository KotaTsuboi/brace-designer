use super::*;
use strum::IntoEnumIterator;

#[tauri::command]
pub fn list_sections() -> Vec<String> {
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
