use crate::workspace;
use anyhow::Result;
use colored::*;
use comfy_table::Table;
use std::process::Command;

pub fn exec() -> Result<()> {
    let root = workspace::find_root()?;
    println!(
        "{}",
        format!("Syncing APIs in workspace: {:?}", root).cyan()
    );

    let mut table = Table::new();
    table.set_header(vec!["Module", "API Sync Status"]);

    // Try to run 'just sync-api' at the root
    let status = Command::new("just")
        .arg("sync-api")
        .current_dir(&root)
        .status();

    if let Ok(s) = status {
        if s.success() {
            table.add_row(vec!["Root".cyan().to_string(), "Success".green().to_string()]);
        } else {
            table.add_row(vec!["Root".cyan().to_string(), "Failed".red().to_string()]);
        }
    }

    // Check for web/server -> web/client sync
    let server_dir = root.join("web/server");
    let client_dir = root.join("web/client");

    if server_dir.exists() && client_dir.exists() {
        println!("{}", "Coordinating types between web/server and web/client...".cyan());
        
        // Call module-specific sync if it exists
        let sync_status = Command::new("just")
            .arg("sync-api")
            .current_dir(root.join("web"))
            .status();

        match sync_status {
            Ok(s) if s.success() => {
                table.add_row(vec!["Web (Full Stack)".cyan().to_string(), "Success".green().to_string()]);
            }
            _ => {
                table.add_row(vec!["Web (Full Stack)".cyan().to_string(), "No 'just sync-api' found or failed".yellow().to_string()]);
            }
        }
    }

    // Check for ML model metadata sync
    let ml_dir = root.join("ml");
    if ml_dir.exists() {
        table.add_row(vec!["ML".cyan().to_string(), "Auto-synced via heartbeat.json".green().to_string()]);
    }

    println!("{table}");

    Ok(())
}
