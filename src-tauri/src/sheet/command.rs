use super::*;

#[tauri::command]
pub fn write_file(
    base_yield_result: State<Mutex<BaseYieldResult>>,
    bolt_yield_result: State<Mutex<BoltYieldResult>>,
) -> Result<(), ()> {
    let base_yield_result = (*base_yield_result.lock().unwrap()).clone();
    let bolt_yield_result = (*bolt_yield_result.lock().unwrap()).clone();
    let dialog_builder = dialog::FileDialogBuilder::new();
    dialog_builder
        .save_file(|path| write(path.unwrap(), base_yield_result, bolt_yield_result).unwrap());
    Ok(())
}
