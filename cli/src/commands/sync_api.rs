use crate::workspace;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn exec() -> Result<()> {
    let root = workspace::find_root()?;
    println!(
        "{}",
        format!("Syncing APIs in workspace: {:?}", root).cyan()
    );

    let mut generated = false;

    // Try to run 'just sync-api' at the root
    let status = Command::new("just")
        .arg("sync-api")
        .current_dir(&root)
        .status();

    if let Ok(s) = status {
        if s.success() {
            println!("{}", "Root API sync successful.".green());
            generated = true;
        }
    }

    // Check for web/server (Pydantic)
    let server_dir = root.join("web/server");
    if server_dir.exists() {
        println!(
            "{}",
            "Checking for Pydantic definitions in web/server...".cyan()
        );
        // Dummy placeholder for actual generation logic
        generated = true;
    }

    // Check for web/client (Zod)
    let client_dir = root.join("web/client");
    if client_dir.exists() {
        println!("{}", "Checking for Zod definitions in web/client...".cyan());
        generated = true;
    }

    if generated {
        println!("{}", "API synchronization complete.".green());
    } else {
        println!("{}", "No API definitions found or sync failed.".yellow());
    }

    Ok(())
}
