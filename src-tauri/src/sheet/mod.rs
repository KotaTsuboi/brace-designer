pub mod command;

use crate::result::*;
use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::api::dialog;
use tauri::{Builder, State, Wry};
use tectonic;

fn get_latex_str(result: BaseYieldResult) -> Result<String, Box<dyn Error>> {
    let mut reg = Handlebars::new();
    reg.register_escape_fn(|str| handlebars::no_escape(str));

    let base_rows = vec![result.to_latex_table_row()];
    let bolt_rows = vec![r#"\midrule V1 & M20 & S10T & 200 & 4 & 200.0 & 100.0 & 0.500 & OK \\"#];
    let gpl_rows = vec![
        r#"\midrule V1 & SS400 & 200 & 9 & 100.0 & 90.0 & 235 & 200.0 & 100.0 & 0.500 & OK \\"#,
    ];
    let welding_rows = vec![r#"\midrule V1 & II & 6 & 235 & 200.0 & 100.0 & 0.500 & OK \\"#];

    let template = include_str!("../../resources/template.tex");

    let latex = reg.render_template(
        template,
        &json!({
            "base_rows": base_rows.join("\n"),
            "bolt_rows": bolt_rows.join("\n"),
            "gpl_rows": gpl_rows.join("\n"),
            "welding_rows": welding_rows.join("\n"),
        }),
    )?;

    Ok(latex)
}

fn get_pdf_data(result: BaseYieldResult) -> Result<Vec<u8>, Box<dyn Error>> {
    let latex = get_latex_str(result)?;
    let pdf = tectonic::latex_to_pdf(latex)?;
    Ok(pdf)
}

pub fn write(file_path: PathBuf, result: BaseYieldResult) -> Result<(), Box<dyn Error>> {
    let pdf = get_pdf_data(result)?;
    let mut writer = BufWriter::new(File::create(file_path)?);
    writer.write_all(&pdf)?;
    writer.flush()?;
    Ok(())
}
