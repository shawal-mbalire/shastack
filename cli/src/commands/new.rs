use crate::workspace;
use anyhow::Result;
use colored::*;
use inquire::MultiSelect;

pub fn exec(name: String) -> Result<()> {
    println!(
        "{}",
        format!("Initializing shastack workspace: {}", name).cyan()
    );

    let options = vec![
        "Web Frontend (Angular)",
        "Web Backend (Flask)",
        "Mobile App (Flutter)",
        "Research (LaTeX)",
        "ML (Python/Notebooks)",
        "Hardware (Firmware)",
    ];

    let selected_features = MultiSelect::new("Select features to enable:", options).prompt()?;

    workspace::init(&name, selected_features)?;

    println!(
        "{}",
        format!("Workspace {} initialized successfully!", name).green()
    );
    Ok(())
}
