use crate::workspace;
use anyhow::Result;
use colored::*;

pub fn exec(mut feature: String) -> Result<()> {
    let root = workspace::find_root()?;

    if feature == "." {
        let current_dir = std::env::current_dir()?;
        feature = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Could not determine current directory name"))?
            .to_string();
    }

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
