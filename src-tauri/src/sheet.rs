use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use tectonic;

const TEMPLATE: &str = r#"
\documentclass[a4paper,xelatex,ja=standard]{bxjsarticle}

\usepackage{booktabs}
\usepackage{fancyhdr}
\usepackage{placeins}
\pagestyle{fancy}
\rhead{Brace Designer}
\cfoot{}

\begin{document}

\section*{ブレースの検討}

\subsection*{降伏耐力の検討}

\subsubsection*{ブレース母材の検討}

\begin{description}
    \item[$A$] 全断面積 [cm2]
    \item[$A_e$] 有効断面積 [cm2]
    \item[$F_y$] 短期許容応力度 [N/mm2]
    \item[$N_y^S$] 短期許容軸力 [kN]
    \item[$N_d^S$] 短期設計軸力 [kN]
    \item[$\gamma$] 検定比
\end{description}

\begin{table}[htbp]
    \begin{tabular}{cccccccccc}
    \toprule
        符号 & 断面 & 材質 & $A$ & $A_e$ & $F_y$ & $N_y^S$ & $N_d^S$ & $\gamma$ & 判定 \\ \midrule
        {{base_rows}}
        \bottomrule
    \end{tabular}
\end{table}

\FloatBarrier

\subsubsection*{ボルトの検討}

\begin{description}
    \item[$q_y^S$] 1本あたり短期許容せん断力 [kN]
    \item[$n$] ボルト本数 [mm]
    \item[$N_y^S$] 短期許容軸力 [kN]
    \item[$N_d^S$] 短期設計軸力 [kN]
    \item[$\gamma$] 検定比
\end{description}

\begin{table}[htbp]
    \begin{tabular}{ccccccccc}
    \toprule
        符号 & 径 & 材質 & $q_y^S$ & $n$ & $N_y^S$ & $N_d^S$ & $\gamma$ & 判定 \\ \midrule
        {{bolt_rows}}
        \bottomrule
    \end{tabular}
\end{table}

\FloatBarrier

\subsubsection*{ガセットプレート母材の検討}

\begin{description}
    \item[$t$] 厚さ [mm]
    \item[$A$] 全断面積 [cm2]
    \item[$A_e$] 有効断面積 [cm2]
    \item[$F_y$] 短期許容応力度 [N/mm2]
    \item[$N_y^S$] 短期許容軸力 [kN]
    \item[$N_d^S$] 短期設計軸力 [kN]
    \item[$\gamma$] 検定比
\end{description}

\begin{table}[htbp]
    \begin{tabular}{ccccccccccc}
    \toprule
        符号 & 材質 & 有効幅 & $t$ & $A$ & $A_e$ & $F_y$ & $N_y^S$ & $N_d^S$ & $\gamma$ & 判定 \\ \midrule
        {{gpl_rows}}
        \bottomrule
    \end{tabular}
\end{table}

\FloatBarrier

\subsubsection*{ガセットプレート溶接部の検討}

\begin{description}
    \item[$S$] 溶接サイズ [mm]
    \item[$F_y$] 短期許容応力度 [N/mm2]
    \item[$N_y^S$] 短期許容軸力 [kN]
    \item[$N_d^S$] 短期設計軸力 [kN]
    \item[$\gamma$] 検定比
\end{description}

\begin{table}[htbp]
    \begin{tabular}{cccccccc}
    \toprule
        符号 & タイプ & $S$ & $F_y$ & $N_y^S$ & $N_d^S$ & $\gamma$ & 判定 \\ \midrule
        {{welding_rows}}
        \bottomrule
    \end{tabular}
\end{table}

\end{document}

\FloatBarrier

"#;

struct BaseYieldResult<'a> {
    name: &'a str,
    section_name: &'a str,
    material_name: &'a str,
    a: f64,
    ae: f64,
    fy: f64,
    ny: f64,
    nd: f64,
    gamma: f64,
    judge: &'a str,
}

impl<'a> BaseYieldResult<'a> {
    fn to_table_row(&self) -> String {
        format!(
            r#"\midrule {} & {} & {} & {} & {} & {} & {} & {} & {} & {} \\"#,
            self.name,
            self.section_name,
            self.material_name,
            self.a,
            self.ae,
            self.fy,
            self.ny,
            self.nd,
            self.gamma,
            self.judge,
        )
    }
}

fn get_latex_str() -> Result<String, Box<dyn Error>> {
    let mut reg = Handlebars::new();
    reg.register_escape_fn(|str| handlebars::no_escape(str));

    let result = BaseYieldResult {
        name: "V1",
        section_name: "L-100x100x10",
        material_name: "SS400",
        a: 100.0,
        ae: 90.0,
        fy: 235.0,
        ny: 200.0,
        nd: 100.0,
        gamma: 0.5,
        judge: "OK",
    };

    let base_rows = vec![result.to_table_row()];
    let bolt_rows = vec![r#"\midrule V1 & M20 & S10T & 200 & 4 & 200.0 & 100.0 & 0.500 & OK \\"#];
    let gpl_rows = vec![
        r#"\midrule V1 & SS400 & 200 & 9 & 100.0 & 90.0 & 235 & 200.0 & 100.0 & 0.500 & OK \\"#,
    ];
    let welding_rows = vec![r#"\midrule V1 & II & 6 & 235 & 200.0 & 100.0 & 0.500 & OK \\"#];

    let latex = reg.render_template(
        TEMPLATE,
        &json!({
            "base_rows": base_rows.join("\n"),
            "bolt_rows": bolt_rows.join("\n"),
            "gpl_rows": gpl_rows.join("\n"),
            "welding_rows": welding_rows.join("\n"),
        }),
    )?;

    Ok(latex)
}

fn get_pdf_data() -> Result<Vec<u8>, Box<dyn Error>> {
    let latex = get_latex_str()?;
    let pdf = tectonic::latex_to_pdf(latex)?;
    Ok(pdf)
}

pub fn write(file_path: String) -> Result<(), Box<dyn Error>> {
    let pdf = get_pdf_data()?;
    let mut writer = BufWriter::new(File::create(file_path)?);
    writer.write_all(&pdf)?;
    writer.flush()?;
    Ok(())
}
