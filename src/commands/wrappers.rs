use anyhow::Result;
use std::process::Command;
use crate::workspace;

pub fn run(feature: String) -> Result<()> {
    exec_just(&feature, "run")
}

pub fn build(feature: String) -> Result<()> {
    exec_just(&feature, "build")
}

pub fn test(feature: String) -> Result<()> {
    exec_just(&feature, "test")
}

fn exec_just(feature: &str, action: &str) -> Result<()> {
    let root = workspace::find_root()?;
    let feature_dir = root.join(feature);

    if !feature_dir.exists() {
        return Err(anyhow::anyhow!("Feature directory {} not found", feature));
    }

    println!("Executing {} {}...", action, feature);

    let status = Command::new("just")
        .arg(action)
        .current_dir(feature_dir)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("just {} failed with status {}", action, status));
    }

    Ok(())
}
