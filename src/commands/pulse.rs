use anyhow::Result;
use colored::*;
use crate::workspace;
use std::fs;

pub fn exec() -> Result<()> {
    let root = workspace::find_root()?;
    println!("{}", format!("Checking workspace pulse: {:?}", root).cyan());

    // Check ML heartbeats
    let ml_heartbeat = root.join("ml/heartbeat.json");
    if ml_heartbeat.exists() {
        let content = fs::read_to_string(&ml_heartbeat)?;
        println!("{}", format!("ML Pulse: {}", content).green());
    } else {
        println!("{}", "ML Pulse: No heartbeat.json found.".yellow());
    }

    // Check Web health (this would normally be an HTTP request)
    println!("{}", "Web Pulse: Use 'sha run web' to check health endpoints.".cyan());

    Ok(())
}
