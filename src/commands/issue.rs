use anyhow::Result;
use colored::*;
use comfy_table::Table;
use crate::commands::IssueAction;
use std::process::Command;

pub fn exec(action: IssueAction) -> Result<()> {
    match action {
        IssueAction::Start { id, description } => {
            let branch_name = format!("issue-{}-{}", id, description.replace(' ', "-").to_lowercase());
            println!("{}", format!("Starting issue {} on branch {}...", id, branch_name).cyan());

            let status = Command::new("git")
                .args(["checkout", "-b", &branch_name])
                .status()?;

            if status.success() {
                println!("{}", format!("Successfully switched to branch {}.", branch_name).green());
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
                return Err(anyhow::anyhow!("Not on an IDD issue branch. Current branch: {}", branch));
            }

            println!("{}", format!("Finishing issue on branch {}...", branch).cyan());
            println!("{}", "Next steps:".yellow());
            println!("1. git push origin {}", branch);
            println!("2. gh pr create --fill");
        }
    }
    Ok(())
}
