use anyhow::Result;
use inquire::MultiSelect;
use crate::workspace;

pub fn exec(name: String) -> Result<()> {
    println!("Initializing shastack workspace: {}", name);

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

    println!("Workspace {} initialized successfully!", name);
    Ok(())
}
