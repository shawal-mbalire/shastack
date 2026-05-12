use crate::workspace;
use anyhow::Result;
use colored::*;
use std::fs;

pub fn exec() -> Result<()> {
    let root = workspace::find_root()?;
    println!(
        "{}",
        format!("Restoring shastack workspace components: {:?}", root).cyan()
    );

    // Get features from config.json
    let config_path = root.join(".sha/config.json");
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    let features = manifest["features"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid config.json"))?;

    for feature in features {
        let feature_name = feature.as_str().unwrap_or("");
        println!("{}", format!("Restoring feature: {}", feature_name).cyan());
        
        // Re-run scaffolding logic. Since write_file is used in scaffold.rs, 
        // it will overwrite/restore missing files.
        workspace::add_feature(&root, feature_name).or_else(|e| {
            if e.to_string().contains("already exists") {
                // If directory exists, we still want to ensure files are there.
                // add_feature calls create_feature_dir which handles scaffolding.
                // We'll call create_feature_dir directly to avoid "feature already exists" error in manifest.
                workspace::create_feature_dir(&root, feature_name)
            } else {
                Err(e)
            }
        })?;
    }

    println!("{}", "Workspace restoration complete.".green());
    Ok(())
}
