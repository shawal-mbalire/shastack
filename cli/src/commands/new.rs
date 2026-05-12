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
        "Landing Page (Angular)",
        "Mobile App (Flutter)",
        "Research (LaTeX)",
        "ML (Python/Notebooks)",
        "Hardware (Arduino/C++)",
        "Hardware (MicroPython/uv)",
        "Hardware (Embedded Rust)",
    ];

    let selected_features = MultiSelect::new("Select features to enable:", options)
        .with_vim_mode(true)
        .with_help_message("↑↓/jk to move, space to select, enter to confirm")
        .prompt()?;

    workspace::init(&name, selected_features)?;

    println!(
        "{}",
        format!("Workspace {} initialized successfully!", name).green()
    );
    Ok(())
}
