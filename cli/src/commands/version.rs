use crate::workspace;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn exec(component: Option<String>) -> Result<()> {
    let root = workspace::find_root()?;
    let mut version = workspace::get_version(&root)?;

    if let Some(comp) = component {
        if comp == "auto" {
            println!("{}", "Calculating next version based on conventional commits...".cyan());
            
            // Get the last tag if it exists
            let last_tag = Command::new("git")
                .args(["describe", "--tags", "--abbrev=0"])
                .output()
                .map(|o| String::from_utf8(o.stdout).unwrap_or_default().trim().to_string())
                .unwrap_or_default();

            let range = if last_tag.is_empty() {
                "HEAD".to_string()
            } else {
                format!("{}..HEAD", last_tag)
            };

            let output = Command::new("git")
                .args(["log", "--pretty=format:%s", &range])
                .output()?;
            
            let log = String::from_utf8(output.stdout)?;
            let commits: Vec<&str> = log.lines().collect();

            let mut bump = "patch";
            for commit in &commits {
                if commit.contains("BREAKING CHANGE:") || commit.contains("!") && commit.contains(":") {
                    bump = "major";
                    break;
                } else if commit.starts_with("feat") {
                    bump = "minor";
                }
            }

            if commits.is_empty() {
                println!("{}", "No new commits found since last tag. Version remains unchanged.".yellow());
                return Ok(());
            }

            println!("{}", format!("Detected bump: {}", bump).cyan());
            match bump {
                "major" => {
                    version.major += 1;
                    version.minor = 0;
                    version.patch = 0;
                }
                "minor" => {
                    version.minor += 1;
                    version.patch = 0;
                }
                _ => {
                    version.patch += 1;
                }
            }
        } else {
            match comp.as_str() {
                "major" => {
                    version.major += 1;
                    version.minor = 0;
                    version.patch = 0;
                }
                "minor" => {
                    version.minor += 1;
                    version.patch = 0;
                }
                "patch" => {
                    version.patch += 1;
                }
                _ => return Err(anyhow::anyhow!("Invalid version component: {}", comp)),
            }
        }
        workspace::set_version(&root, &version)?;
        println!("{}", format!("Updated version to {}", version).green());
    } else {
        println!("{}", format!("Current version: {}", version).cyan());
    }

    Ok(())
}
