use crate::domain::errors::ShaError;
use crate::domain::models::GitCommit;
use crate::domain::ports::GitPort;
use std::process::Command;

pub struct RealGit;

impl GitPort for RealGit {
    fn current_branch(&self) -> Result<String, ShaError> {
        let output = Command::new("git")
            .args(["branch", "--show-current"])
            .output()?;
        if !output.status.success() {
            return Err(ShaError::GitError("Failed to get current branch".to_string()));
        }
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    fn create_branch(&self, name: &str) -> Result<(), ShaError> {
        let status = Command::new("git")
            .args(["checkout", "-b", name])
            .status()?;
        if !status.success() {
            return Err(ShaError::GitError(format!("Failed to create branch {}", name)));
        }
        Ok(())
    }

    fn push_branch(&self, name: &str) -> Result<(), ShaError> {
        let status = Command::new("git")
            .args(["push", "origin", name])
            .status()?;
        if !status.success() {
            return Err(ShaError::GitError(format!("Failed to push branch {}", name)));
        }
        Ok(())
    }

    fn get_log(&self, range: &str) -> Result<Vec<GitCommit>, ShaError> {
        let output = Command::new("git")
            .args(["log", "--pretty=format:%H|%s", range])
            .output()?;
        if !output.status.success() {
            return Err(ShaError::GitError("Failed to get git log".to_string()));
        }
        let log = String::from_utf8(output.stdout)?;
        Ok(log.lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(2, '|').collect();
                if parts.len() == 2 {
                    Some(GitCommit {
                        hash: parts[0].to_string(),
                        message: parts[1].to_string(),
                    })
                } else {
                    None
                }
            })
            .collect())
    }

    fn get_head_hash(&self) -> Result<String, ShaError> {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()?;
        if !output.status.success() {
            return Err(ShaError::GitError("Failed to get HEAD hash".to_string()));
        }
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    fn describe_tags(&self) -> Result<Option<String>, ShaError> {
        let output = Command::new("git")
            .args(["describe", "--tags", "--abbrev=0"])
            .output()?;
        if output.status.success() {
            Ok(Some(String::from_utf8(output.stdout)?.trim().to_string()))
        } else {
            Ok(None)
        }
    }
}
