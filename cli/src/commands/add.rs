use crate::workspace;
use anyhow::Result;
use colored::*;

pub fn exec(feature: String) -> Result<()> {
    let root = workspace::find_root()?;
    println!(
        "{}",
        format!("Adding feature {} to workspace at {:?}", feature, root).cyan()
    );

    workspace::add_feature(&root, &feature)?;

    println!(
        "{}",
        format!("Feature {} added successfully!", feature).green()
    );
    Ok(())
}
