use crate::domain::errors::ShaError;
use crate::domain::models::EnvVar;
use crate::domain::ports::EnvPort;
use std::process::Command;
use std::io::Write;

pub struct RealEnv;

impl EnvPort for RealEnv {
    fn get_env(&self, namespace: &str, key: &str) -> Result<Option<String>, ShaError> {
        let output = Command::new("envchain")
            .args(["--no-require-passphrase", namespace, "printenv", key])
            .output()?;

        if output.status.success() {
            let value = String::from_utf8(output.stdout)?.trim().to_string();
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn set_env(&self, namespace: &str, key: &str, value: &str) -> Result<(), ShaError> {
        let mut child = Command::new("envchain")
            .args(["--set", "--no-require-passphrase", namespace, key])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ShaError::EnvChainError(format!("Failed to run envchain: {}", e)))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(value.as_bytes())?;
            drop(stdin);
        }

        let output = child.wait_with_output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("failed") || stderr.contains("error") {
                return Err(ShaError::EnvChainError(stderr.trim().to_string()));
            }
        }

        Ok(())
    }

    fn list_envs(&self, namespace: &str) -> Result<Vec<EnvVar>, ShaError> {
        let output = Command::new("envchain")
            .args(["--no-require-passphrase", namespace, "printenv"])
            .output()?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout.lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some(EnvVar {
                        key: parts[0].trim().to_string(),
                        value: parts[1].trim().to_string(),
                    })
                } else {
                    None
                }
            })
            .collect())
    }

    fn is_available(&self) -> bool {
        Command::new("envchain")
            .arg("--help")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}
