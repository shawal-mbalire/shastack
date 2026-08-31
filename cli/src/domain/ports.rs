use crate::domain::errors::ShaError;
use crate::domain::models::*;
use std::path::{Path, PathBuf};

/// Port for file system operations
pub trait FileSystemPort {
    fn read_file(&self, path: &Path) -> Result<String, ShaError>;
    fn write_file(&self, path: &Path, content: &str) -> Result<(), ShaError>;
    fn create_dir(&self, path: &Path) -> Result<(), ShaError>;
    fn dir_exists(&self, path: &Path) -> bool;
    fn file_exists(&self, path: &Path) -> bool;
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>, ShaError>;
    fn rename(&self, from: &Path, to: &Path) -> Result<(), ShaError>;
    fn remove_file(&self, path: &Path) -> Result<(), ShaError>;
    fn current_dir(&self) -> Result<PathBuf, ShaError>;
}

/// Port for git operations
pub trait GitPort {
    fn current_branch(&self) -> Result<String, ShaError>;
    fn create_branch(&self, name: &str) -> Result<(), ShaError>;
    fn push_branch(&self, name: &str) -> Result<(), ShaError>;
    fn get_log(&self, range: &str) -> Result<Vec<GitCommit>, ShaError>;
    fn get_head_hash(&self) -> Result<String, ShaError>;
    fn describe_tags(&self) -> Result<Option<String>, ShaError>;
}

/// Port for environment variable management
pub trait EnvPort {
    fn get_env(&self, namespace: &str, key: &str) -> Result<Option<String>, ShaError>;
    fn set_env(&self, namespace: &str, key: &str, value: &str) -> Result<(), ShaError>;
    fn list_envs(&self, namespace: &str) -> Result<Vec<EnvVar>, ShaError>;
    fn is_available(&self) -> bool;
}

/// Port for command execution
pub trait CommandPort {
    fn run_command(&self, program: &str, args: &[&str], cwd: Option<&Path>) -> Result<(bool, String, String), ShaError>;
    fn command_exists(&self, name: &str) -> bool;
}

/// Port for scaffolding projects
pub trait ScaffoldPort {
    fn scaffold_angular(&self, dir: &Path) -> Result<(), ShaError>;
    fn scaffold_python(&self, dir: &Path, deps: &[&str]) -> Result<(), ShaError>;
    fn scaffold_flutter(&self, dir: &Path) -> Result<(), ShaError>;
    fn scaffold_research(&self, dir: &Path) -> Result<(), ShaError>;
    fn write_justfile(&self, dir: &Path, content: &str) -> Result<(), ShaError>;
    fn write_ci_workflow(&self, dir: &Path, content: &str) -> Result<(), ShaError>;
}

/// Port for network/HTTP operations
pub trait HttpPort {
    fn get(&self, url: &str) -> Result<Vec<u8>, ShaError>;
    fn check_latest_version(&self, owner: &str, repo: &str, current: &str) -> Result<Option<String>, ShaError>;
}

/// Port for interactive prompts
pub trait PromptPort {
    fn multi_select(&self, message: &str, options: &[&str]) -> Result<Vec<String>, ShaError>;
    fn confirm(&self, message: &str, default: bool) -> Result<bool, ShaError>;
    fn input(&self, message: &str) -> Result<Option<String>, ShaError>;
}

/// Port for output/display
pub trait DisplayPort {
    fn print_info(&self, msg: &str);
    fn print_success(&self, msg: &str);
    fn print_warning(&self, msg: &str);
    fn print_error(&self, msg: &str);
    fn print_dry_run(&self, msg: &str);
    fn print_table(&self, headers: &[&str], rows: Vec<Vec<String>>);
}
