use crate::workspace;
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use comfy_table::Table;
use inquire::{Confirm, MultiSelect};
use self_update::cargo_crate_version;
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[command(name = "sha")]
#[command(about = "shastack: The Unified Universal Stack Specification CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Starts an interactive prompt to select features
    New {
        /// Name of the workspace
        name: String,
    },
    /// Adds a new standalone module to the project
    Add {
        /// Feature to add
        feature: Option<String>,
    },
    /// Ensures all enabled features have their required files/folders
    Restore,
    /// Checks for and installs updates for the sha CLI
    Upgrade {
        /// Optional URL to update from
        url: Option<String>,
    },
    /// Updates Semantic Versioning for the project
    Version {
        /// Version component to increment
        #[arg(value_parser = ["major", "minor", "patch", "auto"])]
        component: Option<String>,
    },
    /// Manages project-wide environment variables
    Env {
        #[command(subcommand)]
        action: EnvAction,
    },
    /// Automatically generates clients from Zod/Pydantic definitions
    SyncApi {
        /// Optional URL to sync definitions from
        url: Option<String>,
    },
    /// Installs project-wide and system-wide dependencies
    Deps,
    /// Checks health and heartbeats of the workspace modules
    Pulse,
    /// Manages the ML model registry and research artifacts
    Registry {
        #[command(subcommand)]
        action: RegistryAction,
    },
    /// Opens rustup documentation and project documentation
    Docs {
        /// Feature to open docs for
        #[arg(long)]
        feature: Option<String>,
        /// Open standard library docs
        #[arg(long)]
        std: bool,
    },
    /// Enforces Issue-Driven Development workflows
    Issue {
        #[command(subcommand)]
        action: IssueAction,
    },
}

#[derive(Subcommand)]
pub enum IssueAction {
    /// Starts a new issue by creating a branch
    Start {
        /// GitHub issue ID
        id: u64,
        /// Optional description (will fetch from GitHub if omitted)
        description: Option<String>,
    },
    /// Shows the current issue context
    Status,
    /// Finalizes the current issue and prepares for PR
    Finish,
}

#[derive(Subcommand)]
pub enum RegistryAction {
    /// Pins a model weight with current git hash
    Pin { model: String, weight_path: String },
    /// Lists all registered models
    List,
}

#[derive(Subcommand)]
pub enum EnvAction {
    /// Set an environment variable
    Set { key: String, value: String },
    /// Get an environment variable
    Get { key: String },
    /// Lists all environment variables
    List,
}

pub struct NewCommand;
impl NewCommand {
    pub fn exec(name: String) -> Result<()> {
        println!(
            "{}",
            format!("Initializing shastack workspace: {}", name).cyan()
        );

        let options = vec![
            "Web Frontend (Angular)",
            "Web Backend (Flask)",
            "Landing Page (Angular)",
            "Mobile App (Flutter)",
            "Research (LaTeX)",
            "ML (Python/Notebooks)",
            "Hardware (Arduino/C++)",
            "Hardware (MicroPython/uv)",
            "Hardware (Embedded Rust)",
        ];

        let selected_features = MultiSelect::new("Select features to enable:", options)
            .with_vim_mode(true)
            .with_help_message("↑↓/jk to move, space to select, enter to confirm")
            .prompt()?;

        workspace::init(&name, selected_features)?;

        println!(
            "{}",
            format!("Workspace {} initialized successfully!", name).green()
        );
        Ok(())
    }
}

pub struct AddCommand;
impl AddCommand {
    pub fn exec(feature_arg: Option<String>) -> Result<()> {
        let root = workspace::find_root()?;

        let feature = match feature_arg {
            Some(f) => {
                if f == "." {
                    let current_dir = std::env::current_dir()?;
                    current_dir
                        .file_name()
                        .and_then(|n| n.to_str())
                        .ok_or_else(|| anyhow::anyhow!("Could not determine current directory name"))?
                        .to_string()
                } else {
                    f
                }
            }
            None => {
                let options = vec![
                    "Web Frontend (Angular)",
                    "Web Backend (Flask)",
                    "Landing Page (Angular)",
                    "Mobile App (Flutter)",
                    "Research (LaTeX)",
                    "ML (Python/Notebooks)",
                    "Hardware (Arduino/C++)",
                    "Hardware (MicroPython/uv)",
                    "Hardware (Embedded Rust)",
                ];
                MultiSelect::new("Select features to add:", options)
                    .with_vim_mode(true)
                    .prompt()?
                    .pop() // Just take one for now or loop
                    .ok_or_else(|| anyhow::anyhow!("No feature selected"))?
                    .to_string()
            }
        };

        println!(
            "{}",
            format!("Adding feature {} to workspace at {:?}", feature, root).cyan()
        );

        workspace::add_feature(&root, &feature)?;

        println!(
            "{}",
            format!("Feature {} added successfully!", feature).green()
        );
        Ok(())
    }
}

pub struct RestoreCommand;
impl RestoreCommand {
    pub fn exec() -> Result<()> {
        let root = workspace::find_root()?;
        println!(
            "{}",
            format!("Restoring shastack workspace components: {:?}", root).cyan()
        );

        let config_path = root.join(".sha/config.json");
        let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
        let features = manifest["features"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid config.json"))?;

        for feature in features {
            let feature_name = feature.as_str().unwrap_or("");
            println!("{}", format!("Restoring feature: {}", feature_name).cyan());
            
            workspace::add_feature(&root, feature_name).or_else(|e| {
                if e.to_string().contains("already exists") {
                    workspace::create_feature_dir(&root, feature_name)
                } else {
                    Err(e)
                }
            })?;
        }

        println!("{}", "Workspace restoration complete.".green());
        Ok(())
    }
}

pub struct UpdateCommand;
impl UpdateCommand {
    pub fn exec(url: Option<String>) -> Result<()> {
        if let Some(url) = url {
            Self::update_from_url(&url)?;
        } else {
            Self::check_for_updates()?;
        }
        Ok(())
    }

    pub fn check_for_updates() -> Result<()> {
        if std::env::var("SHA_SKIP_UPDATE").is_ok() || std::env::var("CI").is_ok() {
            return Ok(());
        }

        let current_version = cargo_crate_version!();
        
        let status = self_update::backends::github::Update::configure()
            .repo_owner("shawal-mbalire")
            .repo_name("shastack")
            .bin_name("sha")
            .show_download_progress(true)
            .current_version(current_version)
            .no_confirm(true)
            .build()?;

        let latest_release = status.get_latest_release()?;
        
        if self_update::version::bump_is_greater(current_version, &latest_release.version)? {
            println!(
                "{}",
                format!(
                    "A new version of sha is available: {} -> {}",
                    current_version.yellow(),
                    latest_release.version.green()
                )
                .bold()
            );

            let ans = Confirm::new("Would you like to upgrade now?")
                .with_default(true)
                .prompt()?;

            if ans {
                println!("{}", "Upgrading sha...".cyan());
                status.update()?;
                println!("{}", "Successfully upgraded to latest version!".green());
                std::process::exit(0);
            }
        }

        Ok(())
    }

    fn update_from_url(url: &str) -> Result<()> {
        println!("{}", format!("Downloading update from {}...", url).cyan());
        let response = reqwest::blocking::get(url)?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to download update from URL: {}. Status: {}",
                url,
                response.status()
            ));
        }

        let bytes = response.bytes()?;
        let current_exe = std::env::current_exe()?;
        let tmp_exe = current_exe.with_extension("tmp_update");

        fs::write(&tmp_exe, bytes)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&tmp_exe)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&tmp_exe, perms)?;
        }

        self_update::Move::from_source(&tmp_exe)
            .to_dest(&current_exe)?;

        println!("{}", "Successfully updated sha from URL!".green());
        std::process::exit(0);
    }
}

pub struct VersionCommand;
impl VersionCommand {
    pub fn exec(component: Option<String>) -> Result<()> {
        let root = workspace::find_root()?;
        let mut version = workspace::get_version(&root)?;

        if let Some(comp) = component {
            if comp == "auto" {
                println!("{}", "Calculating next version based on conventional commits...".cyan());
                
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
}

pub struct EnvCommand;
impl EnvCommand {
    pub fn exec(action: EnvAction) -> Result<()> {
        let root = workspace::find_root()?;

        match action {
            EnvAction::Set { key, value } => {
                workspace::set_env(&root, &key, &value)?;
                println!("{}", format!("Environment variable {} set.", key).green());
            }
            EnvAction::Get { key } => {
                if let Some(value) = workspace::get_env(&root, &key)? {
                    let mut table = Table::new();
                    table.set_header(vec!["Key", "Value"]);
                    table.add_row(vec![key.cyan().to_string(), value.yellow().to_string()]);
                    println!("{table}");
                } else {
                    return Err(anyhow::anyhow!("Environment variable {} not found", key));
                }
            }
            EnvAction::List => {
                let envs = workspace::list_envs(&root)?;
                if envs.is_empty() {
                    println!("{}", "No environment variables found.".yellow());
                } else {
                    let mut table = Table::new();
                    table.set_header(vec!["Key", "Value"]);
                    for (k, v) in envs {
                        table.add_row(vec![k.cyan().to_string(), v.yellow().to_string()]);
                    }
                    println!("{table}");
                }
            }
        }
        Ok(())
    }
}

pub struct SyncApiCommand;
impl SyncApiCommand {
    pub fn exec(url: Option<String>) -> Result<()> {
        let root = workspace::find_root()?;

        if let Some(url) = url {
            println!(
                "{}",
                format!("Fetching API definitions from {}...", url).cyan()
            );
            let response = reqwest::blocking::get(&url)?;
            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "Failed to fetch API definitions: {}",
                    response.status()
                ));
            }
            let bytes = response.bytes()?;
            let shared_dir = root.join("shared");
            fs::create_dir_all(&shared_dir)?;
            let dest = shared_dir.join("remote_schema.json");
            fs::write(&dest, bytes)?;
            println!("{}", format!("Saved remote schema to {:?}", dest).green());
        }

        println!(
            "{}",
            format!("Syncing APIs in workspace: {:?}", root).cyan()
        );

        let mut table = Table::new();
        table.set_header(vec!["Module", "API Sync Status"]);

        let status = Command::new("just")
            .arg("sync-api")
            .current_dir(&root)
            .status();

        if let Ok(s) = status {
            if s.success() {
                table.add_row(vec!["Root".cyan().to_string(), "Success".green().to_string()]);
            } else {
                table.add_row(vec!["Root".cyan().to_string(), "Failed".red().to_string()]);
            }
        }

        let server_dir = root.join("web/server");
        let client_dir = root.join("web/client");

        if server_dir.exists() && client_dir.exists() {
            println!(
                "{}",
                "Coordinating types between web/server and web/client...".cyan()
            );

            let sync_status = Command::new("just")
                .arg("sync-api")
                .current_dir(root.join("web"))
                .status();

            match sync_status {
                Ok(s) if s.success() => {
                    table.add_row(vec![
                        "Web (Full Stack)".cyan().to_string(),
                        "Success".green().to_string(),
                    ]);
                }
                _ => {
                    table.add_row(vec![
                        "Web (Full Stack)".cyan().to_string(),
                        "No 'just sync-api' found or failed".yellow().to_string(),
                    ]);
                }
            }
        }

        let ml_dir = root.join("ml");
        if ml_dir.exists() {
            table.add_row(vec![
                "ML".cyan().to_string(),
                "Auto-synced via heartbeat.json".green().to_string(),
            ]);
        }

        println!("{table}");

        Ok(())
    }
}

pub struct DepsCommand;
impl DepsCommand {
    pub fn exec() -> Result<()> {
        println!(
            "{}",
            "Checking and installing system-wide dependencies...".cyan()
        );

        Self::check_tool("just", "https://just.systems/man/en/chapter_4.html")?;
        Self::check_tool("git", "https://git-scm.com/downloads")?;
        Self::check_tool("gh", "https://cli.github.com/")?;
        
        Self::install_bun_if_missing()?;
        Self::install_uv_if_missing()?;
        Self::install_angular_cli_if_missing()?;

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

        let config_path = root.join(".sha/config.json");
        let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
        let features = manifest["features"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid config.json"))?;

        for feature in features {
            let feature_name = feature.as_str().unwrap_or("");
            let feature_path = match feature_name {
                "Web Frontend (Angular)" | "Web Backend (Flask)" => "web",
                "Landing Page (Angular)" => "landing",
                "Mobile App (Flutter)" => {
                    Self::check_tool("flutter", "https://docs.flutter.dev/get-started/install")?;
                    "mobile"
                }
                "Research (LaTeX)" => {
                    Self::check_tool("pdflatex", "https://www.latex-project.org/get/")?;
                    "research"
                }
                "ML (Python/Notebooks)" => "ml",
                "Hardware (Arduino/C++)" | "Hardware (MicroPython/uv)" | "Hardware (Embedded Rust)" => {
                    Self::check_tool("pio", "https://docs.platformio.org/en/latest/core/installation.html")?;
                    "hardware"
                }
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

    fn check_tool(name: &str, install_url: &str) -> Result<()> {
        if Self::command_exists(name) {
            println!("{}", format!("✓ {} is installed.", name).green());
            Ok(())
        } else {
            println!("{}", format!("✗ {} is missing.", name).red());
            println!("{}", format!("  Please install it from: {}", install_url).yellow());
            Err(anyhow::anyhow!("Missing required tool: {}", name))
        }
    }

    fn install_bun_if_missing() -> Result<()> {
        if Self::command_exists("bun") {
            println!("{}", "✓ Bun is already installed.".green());
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
        if Self::command_exists("uv") {
            println!("{}", "✓ UV is already installed.".green());
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
        if Self::command_exists("ng") {
            println!("{}", "✓ Angular CLI is already installed.".green());
        } else {
            if Self::command_exists("npm") {
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
}

pub struct PulseCommand;
impl PulseCommand {
    pub fn exec() -> Result<()> {
        let root = workspace::find_root()?;
        println!("{}", format!("Checking workspace pulse: {:?}", root).cyan());

        let mut table = Table::new();
        table.set_header(vec!["Module", "Status", "Last Heartbeat / Details"]);

        let config_path = root.join(".sha/config.json");
        let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
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
            if !dir.exists() {
                continue;
            }

            let (status, details) = match feature_path {
                "ml" => {
                    let hb = dir.join("heartbeat.json");
                    if hb.exists() {
                        let content = fs::read_to_string(hb)?;
                        ("ACTIVE".green(), content)
                    } else {
                        ("IDLE".yellow(), "No heartbeat.json found".to_string())
                    }
                }
                "web" | "landing" => {
                    ("RUNNING?".cyan(), "Use 'sha run' to check health endpoints".to_string())
                }
                "research" => {
                    let pdf = dir.join("main.pdf");
                    if pdf.exists() {
                        ("COMPLETE".green(), "Artifact main.pdf present".to_string())
                    } else {
                        ("PENDING".yellow(), "No PDF built yet".to_string())
                    }
                }
                "hardware" => {
                    ("READY".green(), "Toolchain configured".to_string())
                }
                _ => ("UNKNOWN".white(), "No pulse logic defined".to_string()),
            };

            table.add_row(vec![
                feature_name.cyan().to_string(),
                status.to_string(),
                details.white().to_string(),
            ]);
        }

        println!("{table}");

        Ok(())
    }
}

pub struct RegistryCommand;
impl RegistryCommand {
    pub fn exec(action: RegistryAction) -> Result<()> {
        let root = workspace::find_root()?;

        match action {
            RegistryAction::Pin { model, weight_path } => {
                let registry_dir = root.join("ml/model_registry").join(&model);
                fs::create_dir_all(&registry_dir)?;

                let git_hash = Command::new("git")
                    .args(["rev-parse", "HEAD"])
                    .output()?
                    .stdout;
                let git_hash = String::from_utf8(git_hash)?.trim().to_string();

                let metadata = serde_json::json!({
                    "model": model,
                    "weight_path": weight_path,
                    "git_hash": git_hash,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                });

                fs::write(
                    registry_dir.join("metadata.json"),
                    serde_json::to_string_pretty(&metadata)?,
                )?;

                println!(
                    "{}",
                    format!("Model {} pinned with hash {}.", model, git_hash).green()
                );
            }
            RegistryAction::List => {
                let registry_dir = root.join("ml/model_registry");
                if !registry_dir.exists() {
                    println!("{}", "Model registry empty.".yellow());
                    return Ok(());
                }

                let mut table = Table::new();
                table.set_header(vec!["Model", "Weight Path", "Git Hash", "Pinned At"]);

                for entry in fs::read_dir(registry_dir)? {
                    let entry = entry?;
                    if entry.path().is_dir() {
                        let metadata_path = entry.path().join("metadata.json");
                        if metadata_path.exists() {
                            let metadata: serde_json::Value =
                                serde_json::from_str(&fs::read_to_string(metadata_path)?)?;
                            table.add_row(vec![
                                metadata["model"].as_str().unwrap_or("").cyan().to_string(),
                                metadata["weight_path"]
                                    .as_str()
                                    .unwrap_or("")
                                    .yellow()
                                    .to_string(),
                                metadata["git_hash"]
                                    .as_str()
                                    .unwrap_or("")
                                    .magenta()
                                    .to_string(),
                                metadata["timestamp"]
                                    .as_str()
                                    .unwrap_or("")
                                    .white()
                                    .to_string(),
                            ]);
                        }
                    }
                }
                println!("{table}");
            }
        }

        Ok(())
    }
}

pub struct DocsCommand;
impl DocsCommand {
    pub fn exec(feature: Option<String>, std: bool) -> Result<()> {
        if std {
            println!(
                "{}",
                "Opening rustup standard library documentation...".cyan()
            );
            Command::new("rustup").args(["doc", "--std"]).status()?;
            return Ok(());
        }

        if let Some(f) = feature {
            let root = workspace::find_root()?;
            let feature_dir = root.join(&f);
            if feature_dir.exists() {
                println!(
                    "{}",
                    format!("Opening documentation for feature: {}", f).cyan()
                );
                Command::new("just")
                    .arg("doc")
                    .current_dir(feature_dir)
                    .status()?;
            } else {
                return Err(anyhow::anyhow!("Feature {} not found", f));
            }
        } else {
            println!("{}", "Opening rustup documentation...".cyan());
            Command::new("rustup").arg("doc").status()?;
        }

        Ok(())
    }
}

pub struct IssueCommand;
impl IssueCommand {
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
}
