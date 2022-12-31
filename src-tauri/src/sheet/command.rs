use super::*;

#[tauri::command]
pub fn write_file(result: State<Mutex<BaseYieldResult>>) -> Result<(), ()> {
    let result = (*result.lock().unwrap()).clone();
    let dialog_builder = dialog::FileDialogBuilder::new();
    dialog_builder.save_file(|path| write(path.unwrap(), result).unwrap());
    Ok(())
}
