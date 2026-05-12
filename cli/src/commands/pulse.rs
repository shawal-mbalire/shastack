use crate::workspace;
use anyhow::Result;
use colored::*;
use comfy_table::Table;
use std::fs;

pub fn exec() -> Result<()> {
    let root = workspace::find_root()?;
    println!("{}", format!("Checking workspace pulse: {:?}", root).cyan());

    let mut table = Table::new();
    table.set_header(vec!["Module", "Status", "Last Heartbeat / Details"]);

    // Get features from config.json
    let config_path = root.join(".sha/config.json");
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    let features = manifest["features"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid config.json"))?;

    for feature in features {
        let feature_name = feature.as_str().unwrap_or("");
        let feature_path = match feature_name {
            "Web Frontend (Angular)" | "Web Backend (Flask)" => "web",
            "Landing Page (Angular)" => "landing",
            "Mobile App (Flutter)" => "mobile",
            "Research (LaTeX)" => "research",
            "ML (Python/Notebooks)" => "ml",
            "Hardware (Arduino/C++)" | "Hardware (MicroPython/uv)" | "Hardware (Embedded Rust)" => "hardware",
            _ => feature_name,
        };

        let dir = root.join(feature_path);
        if !dir.exists() {
            continue;
        }

        let (status, details) = match feature_path {
            "ml" => {
                let hb = dir.join("heartbeat.json");
                if hb.exists() {
                    let content = fs::read_to_string(hb)?;
                    ("ACTIVE".green(), content)
                } else {
                    ("IDLE".yellow(), "No heartbeat.json found".to_string())
                }
            }
            "web" | "landing" => {
                ("RUNNING?".cyan(), "Use 'sha run' to check health endpoints".to_string())
            }
            "research" => {
                let pdf = dir.join("main.pdf");
                if pdf.exists() {
                    ("COMPLETE".green(), "Artifact main.pdf present".to_string())
                } else {
                    ("PENDING".yellow(), "No PDF built yet".to_string())
                }
            }
            "hardware" => {
                ("READY".green(), "Toolchain configured".to_string())
            }
            _ => ("UNKNOWN".white(), "No pulse logic defined".to_string()),
        };

        table.add_row(vec![
            feature_name.cyan().to_string(),
            status.to_string(),
            details.white().to_string(),
        ]);
    }

    println!("{table}");

    Ok(())
}
