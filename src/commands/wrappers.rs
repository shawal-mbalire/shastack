use anyhow::Result;
use colored::*;
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

pub fn flash() -> Result<()> {
    exec_just("hardware", "flash")
}

pub fn deploy(feature: String, target: String) -> Result<()> {
    let root = workspace::find_root()?;
    let feature_dir = root.join(&feature);

    if !feature_dir.exists() {
        return Err(anyhow::anyhow!("Feature directory {} not found", feature));
    }

    println!("{}", format!("Deploying {} to {}...", feature, target).cyan());

    let status = Command::new("just")
        .arg("deploy")
        .arg(target)
        .current_dir(feature_dir)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("just deploy failed with status {}", status));
    }

    Ok(())
}

fn exec_just(feature: &str, action: &str) -> Result<()> {
    let root = workspace::find_root()?;
    let feature_dir = root.join(feature);

    if !feature_dir.exists() {
        return Err(anyhow::anyhow!("Feature directory {} not found", feature));
    }

    println!("{}", format!("Executing {} {}...", action, feature).cyan());

    let status = Command::new("just")
        .arg(action)
        .current_dir(feature_dir)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("just {} failed with status {}", action, status));
    }

    Ok(())
}
