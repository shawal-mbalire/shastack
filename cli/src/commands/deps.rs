use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn exec() -> Result<()> {
    println!("{}", "Checking and installing system-wide dependencies...".cyan());

    install_bun_if_missing()?;
    install_uv_if_missing()?;
    install_angular_cli_if_missing()?;

    println!("{}", "All requested tools are present.".green());
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
            .args(["-c", &format!("Get-Command {} -ErrorAction SilentlyContinue", cmd)])
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
