use anyhow::Result;
use crate::workspace;
use std::fs;

pub fn exec() -> Result<()> {
    let root = workspace::find_root()?;
    println!("Checking workspace pulse: {:?}", root);

    // Check ML heartbeats
    let ml_heartbeat = root.join("ml/heartbeat.json");
    if ml_heartbeat.exists() {
        let content = fs::read_to_string(&ml_heartbeat)?;
        println!("ML Pulse: {}", content);
    } else {
        println!("ML Pulse: No heartbeat.json found.");
    }

    // Check Web health (this would normally be an HTTP request)
    println!("Web Pulse: Use 'sha run web' to check health endpoints.");

    Ok(())
}
