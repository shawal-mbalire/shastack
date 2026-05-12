use crate::commands::IssueAction;
use anyhow::Result;
use colored::*;
use comfy_table::Table;
use std::process::Command;

pub fn exec(action: IssueAction) -> Result<()> {
    match action {
        IssueAction::Start { id, description } => {
            let desc = match description {
                Some(d) => d,
                None => {
                    println!("{}", "Fetching issue title from GitHub...".cyan());
                    let output = Command::new("gh")
                        .args(["issue", "view", &id.to_string(), "--json", "title", "-q", ".title"])
                        .output()?;
                    if !output.status.success() {
                        return Err(anyhow::anyhow!("Failed to fetch issue from GitHub. Ensure 'gh' is installed and authenticated."));
                    }
                    String::from_utf8(output.stdout)?.trim().to_string()
                }
            };

            let branch_name = format!(
                "issue-{}-{}",
                id,
                desc.replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "").to_lowercase()
            );
            println!(
                "{}",
                format!("Starting issue {} on branch {}...", id, branch_name).cyan()
            );

            let status = Command::new("git")
                .args(["checkout", "-b", &branch_name])
                .status()?;

            if status.success() {
                println!(
                    "{}",
                    format!("Successfully switched to branch {}.", branch_name).green()
                );
            } else {
                return Err(anyhow::anyhow!("Failed to create branch {}.", branch_name));
            }
        }
        IssueAction::Status => {
            let output = Command::new("git")
                .args(["branch", "--show-current"])
                .output()?;
            let branch = String::from_utf8(output.stdout)?.trim().to_string();

            let mut table = Table::new();
            table.set_header(vec!["Current Branch", "IDD Status"]);

            if branch.starts_with("issue-") {
                table.add_row(vec![
                    branch.cyan().to_string(),
                    "IDD Compliant".green().to_string(),
                ]);
            } else {
                table.add_row(vec![
                    branch.red().to_string(),
                    "NON-IDD COMPLIANT".red().bold().to_string(),
                ]);
            }
            println!("{table}");
        }
        IssueAction::Finish => {
            let output = Command::new("git")
                .args(["branch", "--show-current"])
                .output()?;
            let branch = String::from_utf8(output.stdout)?.trim().to_string();

            if !branch.starts_with("issue-") {
                return Err(anyhow::anyhow!(
                    "Not on an IDD issue branch. Current branch: {}",
                    branch
                ));
            }

            println!(
                "{}",
                format!("Finishing issue on branch {}...", branch).cyan()
            );
            
            println!("{}", "Pushing branch to origin...".cyan());
            Command::new("git")
                .args(["push", "origin", &branch])
                .status()?;

            println!("{}", "Creating Pull Request...".cyan());
            let status = Command::new("gh")
                .args(["pr", "create", "--fill"])
                .status()?;

            if status.success() {
                println!("{}", "Issue finished and PR created successfully!".green());
            } else {
                println!("{}", "PR creation failed. You might need to do it manually.".yellow());
            }
        }
    }
    Ok(())
}
