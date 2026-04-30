use anyhow::Result;
use colored::*;
use comfy_table::Table;
use crate::commands::RegistryAction;
use crate::workspace;
use std::fs;
use std::process::Command;

pub fn exec(action: RegistryAction) -> Result<()> {
    let root = workspace::find_root()?;

    match action {
        RegistryAction::Pin { model, weight_path } => {
            let registry_dir = root.join("ml/model_registry").join(&model);
            fs::create_dir_all(&registry_dir)?;

            let git_hash = Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output()?
                .stdout;
            let git_hash = String::from_utf8(git_hash)?.trim().to_string();

            let metadata = serde_json::json!({
                "model": model,
                "weight_path": weight_path,
                "git_hash": git_hash,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            fs::write(
                registry_dir.join("metadata.json"),
                serde_json::to_string_pretty(&metadata)?,
            )?;

            println!("{}", format!("Model {} pinned with hash {}.", model, git_hash).green());
        }
        RegistryAction::List => {
            let registry_dir = root.join("ml/model_registry");
            if !registry_dir.exists() {
                println!("{}", "Model registry empty.".yellow());
                return Ok(());
            }

            let mut table = Table::new();
            table.set_header(vec!["Model", "Weight Path", "Git Hash", "Pinned At"]);

            for entry in fs::read_dir(registry_dir)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    let metadata_path = entry.path().join("metadata.json");
                    if metadata_path.exists() {
                        let metadata: serde_json::Value = serde_json::from_str(&fs::read_to_string(metadata_path)?)?;
                        table.add_row(vec![
                            metadata["model"].as_str().unwrap_or("").cyan().to_string(),
                            metadata["weight_path"].as_str().unwrap_or("").yellow().to_string(),
                            metadata["git_hash"].as_str().unwrap_or("").magenta().to_string(),
                            metadata["timestamp"].as_str().unwrap_or("").white().to_string(),
                        ]);
                    }
                }
            }
            println!("{table}");
        }
    }

    Ok(())
}
