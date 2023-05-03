use super::*;

#[tauri::command]
pub fn write_file(brace_result: State<Mutex<BraceResult>>) -> Result<(), ()> {
    let brace_result = (*brace_result.lock().unwrap()).clone();
    let dialog_builder = dialog::FileDialogBuilder::new();
    dialog_builder.save_file(|path| write(path.unwrap(), brace_result).unwrap());
    Ok(())
}
