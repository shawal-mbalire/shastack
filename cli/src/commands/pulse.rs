use crate::workspace;
use anyhow::Result;
use colored::*;
use comfy_table::Table;
use std::fs;

pub fn exec() -> Result<()> {
    let root = workspace::find_root()?;
    println!("{}", format!("Checking workspace pulse: {:?}", root).cyan());

    let mut table = Table::new();
    table.set_header(vec!["Module", "Pulse / Status"]);

    // Check ML heartbeats
    let ml_heartbeat = root.join("ml/heartbeat.json");
    let ml_status = if ml_heartbeat.exists() {
        let content = fs::read_to_string(&ml_heartbeat)?;
        content.green().to_string()
    } else {
        "No heartbeat.json found.".yellow().to_string()
    };
    table.add_row(vec!["ML".cyan().to_string(), ml_status]);

    // Check Web health
    table.add_row(vec![
        "Web".cyan().to_string(),
        "Use 'sha run web' for health endpoints."
            .white()
            .to_string(),
    ]);

    println!("{table}");

    Ok(())
}
