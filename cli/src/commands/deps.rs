use crate::workspace;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn exec() -> Result<()> {
    println!(
        "{}",
        "Checking and installing system-wide dependencies...".cyan()
    );

    install_bun_if_missing()?;
    install_uv_if_missing()?;
    install_angular_cli_if_missing()?;

    println!("{}", "System-wide tools are present.".green());

    let root = match workspace::find_root() {
        Ok(r) => r,
        Err(_) => {
            println!("{}", "Not in a shastack workspace, skipping project dependencies.".yellow());
            return Ok(());
        }
    };

    println!(
        "{}",
        format!("Installing project dependencies in workspace: {:?}", root).cyan()
    );

    // Get features from config.json
    let config_path = root.join(".sha/config.json");
    let manifest: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(config_path)?)?;
    let features = manifest["features"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid config.json"))?;

    for feature in features {
        let feature_name = feature.as_str().unwrap_or("");
        let feature_path = match feature_name {
            "Web Frontend (Angular)" | "Web Backend (Flask)" => "web",
            "Landing Page (Angular)" => "landing",
            "Mobile App (Flutter)" => "mobile",
            "Research (LaTeX)" => "research",
            "ML (Python/Notebooks)" => "ml",
            "Hardware (Arduino/C++)" | "Hardware (MicroPython/uv)" | "Hardware (Embedded Rust)" => "hardware",
            _ => feature_name,
        };

        let dir = root.join(feature_path);
        if dir.exists() && dir.join("justfile").exists() {
            println!("{}", format!("Installing dependencies for {}...", feature_name).cyan());
            let status = Command::new("just")
                .arg("deps")
                .current_dir(&dir)
                .status();
            
            if let Ok(s) = status {
                if s.success() {
                    println!("{}", format!("Dependencies for {} installed successfully.", feature_name).green());
                } else {
                    println!("{}", format!("Failed to install dependencies for {}.", feature_name).red());
                }
            }
        }
    }

    Ok(())
}

fn install_bun_if_missing() -> Result<()> {
    if command_exists("bun") {
        println!("{}", "Bun is already installed.".green());
    } else {
        println!("{}", "Installing Bun...".yellow());
        let status = if cfg!(windows) {
            Command::new("powershell")
                .args(["-c", "irm https://bun.sh/install.ps1 | iex"])
                .status()?
        } else {
            Command::new("sh")
                .args(["-c", "curl -fsSL https://bun.sh/install | bash"])
                .status()?
        };
        if !status.success() {
            println!("{}", "Failed to install Bun automatically. Please install it manually from https://bun.sh/".red());
        }
    }
    Ok(())
}

fn install_uv_if_missing() -> Result<()> {
    if command_exists("uv") {
        println!("{}", "UV is already installed.".green());
    } else {
        println!("{}", "Installing UV...".yellow());
        let status = if cfg!(windows) {
            Command::new("powershell")
                .args(["-c", "irm https://astral.sh/uv/install.ps1 | iex"])
                .status()?
        } else {
            Command::new("sh")
                .args(["-c", "curl -LsSf https://astral.sh/uv/install.sh | sh"])
                .status()?
        };
        if !status.success() {
            println!("{}", "Failed to install UV automatically. Please install it manually from https://github.com/astral-sh/uv".red());
        }
    }
    Ok(())
}

fn install_angular_cli_if_missing() -> Result<()> {
    if command_exists("ng") {
        println!("{}", "Angular CLI is already installed.".green());
    } else {
        if command_exists("npm") {
            println!("{}", "Installing Angular CLI via npm...".yellow());
            let status = Command::new("npm")
                .args(["install", "-g", "@angular/cli"])
                .status()?;
            if !status.success() {
                println!("{}", "Failed to install Angular CLI. Ensure you have permissions or install it manually.".red());
            }
        } else {
            println!("{}", "npm not found. Skipping Angular CLI installation. Please install Node.js/npm first.".yellow());
        }
    }
    Ok(())
}

fn command_exists(cmd: &str) -> bool {
    if cfg!(windows) {
        Command::new("powershell")
            .args([
                "-c",
                &format!("Get-Command {} -ErrorAction SilentlyContinue", cmd),
            ])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    } else {
        Command::new("sh")
            .args(["-c", &format!("command -v {}", cmd)])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
