use crate::domain::errors::ShaError;
use crate::domain::ports::CommandPort;
use std::path::Path;
use std::process::Command;

pub struct RealCommand;

impl CommandPort for RealCommand {
    fn run_command(&self, program: &str, args: &[&str], cwd: Option<&Path>) -> Result<(bool, String, String), ShaError> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let output = cmd.output()?;
        let stdout = String::from_utf8(output.stdout).unwrap_or_default();
        let stderr = String::from_utf8(output.stderr).unwrap_or_default();
        Ok((output.status.success(), stdout, stderr))
    }

    fn command_exists(&self, name: &str) -> bool {
        if cfg!(windows) {
            Command::new("powershell")
                .args(["-c", &format!("Get-Command {} -ErrorAction SilentlyContinue", name)])
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        } else {
            Command::new("sh")
                .args(["-c", &format!("command -v {}", name)])
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
    }
}
