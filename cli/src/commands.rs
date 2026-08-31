use crate::domain::models::*;
use crate::domain::ports::*;
use crate::domain::use_cases::WorkspaceUseCases;
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use comfy_table::Table;
use self_update::cargo_crate_version;

#[derive(Parser)]
#[command(name = "sha")]
#[command(about = "shastack: The Unified Universal Stack Specification CLI", long_about = None)]
pub struct Cli {
    /// Preview what would happen without making changes
    #[arg(long, global = true)]
    pub dry_run: bool,

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
    /// Manages project-wide environment variables via envchain (keychain-backed, no .env files)
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
    pub fn exec(
        name: String,
        dry_run: bool,
        use_cases: &WorkspaceUseCases,
        prompt: &dyn PromptPort,
    ) -> Result<()> {
        println!(
            "{}",
            format!("Initializing shastack workspace: {}", name).cyan()
        );

        if dry_run {
            use_cases.init_workspace(&name, vec![], true)?;
            return Ok(());
        }

        let options = FeatureKind::all_names();
        let selected = prompt.multi_select("Select features to enable:", &options)?;
        let feature_refs: Vec<&str> = selected.iter().map(|s| s.as_str()).collect();

        use_cases.init_workspace(&name, feature_refs, false)?;

        println!(
            "{}",
            format!("Workspace {} initialized successfully!", name).green()
        );
        Ok(())
    }
}

pub struct AddCommand;
impl AddCommand {
    pub fn exec(
        feature_arg: Option<String>,
        dry_run: bool,
        use_cases: &WorkspaceUseCases,
        prompt: &dyn PromptPort,
    ) -> Result<()> {
        let root = use_cases.find_root()?;

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
                if dry_run {
                    println!("{}", "Would select feature interactively".yellow());
                    println!("{}", format!("Would add feature to workspace at {:?}", root).yellow());
                    println!("{}", "\nNo changes made (dry-run mode).".green());
                    return Ok(());
                }
                let options = FeatureKind::all_names();
                let selected = prompt.multi_select("Select features to add:", &options)?;
                selected.first()
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("No feature selected"))?
            }
        };

        use_cases.add_feature(&root, &feature, dry_run)?;
        Ok(())
    }
}

pub struct RestoreCommand;
impl RestoreCommand {
    pub fn exec(dry_run: bool, use_cases: &WorkspaceUseCases) -> Result<()> {
        let root = use_cases.find_root()?;
        println!(
            "{}",
            format!("Restoring shastack workspace components: {:?}", root).cyan()
        );

        use_cases.restore_workspace(&root, dry_run)?;
        Ok(())
    }
}

pub struct UpdateCommand;
impl UpdateCommand {
    pub fn exec(url: Option<String>, http: &dyn HttpPort, display: &dyn DisplayPort) -> Result<()> {
        if let Some(url) = url {
            Self::update_from_url(&url, http, display)?;
        } else {
            Self::check_for_updates(http, display)?;
        }
        Ok(())
    }

    pub fn check_for_updates(http: &dyn HttpPort, display: &dyn DisplayPort) -> Result<()> {
        if std::env::var("SHA_SKIP_UPDATE").is_ok() || std::env::var("CI").is_ok() {
            return Ok(());
        }

        let current_version = cargo_crate_version!();

        match http.check_latest_version("shawal-mbalire", "shastack", current_version) {
            Ok(Some(new_version)) => {
                display.print_warning(&format!(
                    "A new version of sha is available: {} -> {}",
                    current_version.yellow(),
                    new_version.green()
                ));

                // In a real implementation, we'd use a prompt adapter here
                println!("Would you like to upgrade now? (y/n)");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                    display.print_info("Upgrading sha...");
                    // Trigger upgrade via self_update
                    let status = self_update::backends::github::Update::configure()
                        .repo_owner("shawal-mbalire")
                        .repo_name("shastack")
                        .bin_name("sha")
                        .show_download_progress(true)
                        .current_version(current_version)
                        .no_confirm(true)
                        .build()?;
                    status.update()?;
                    display.print_success("Successfully upgraded to latest version!");
                    std::process::exit(0);
                } else {
                    display.print_warning("Upgrade skipped.");
                }
            }
            Ok(None) => {
                display.print_success(&format!("sha is already up to date (v{}).", current_version));
            }
            Err(e) => {
                display.print_warning(&format!("Could not check for updates: {}", e));
            }
        }

        Ok(())
    }

    fn update_from_url(url: &str, http: &dyn HttpPort, display: &dyn DisplayPort) -> Result<()> {
        display.print_info(&format!("Downloading update from {}...", url));
        let bytes = http.get(url)?;

        let current_exe = std::env::current_exe()?;
        let tmp_exe = current_exe.with_extension("tmp_update");

        std::fs::write(&tmp_exe, bytes)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&tmp_exe)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&tmp_exe, perms)?;
        }

        self_update::Move::from_source(&tmp_exe)
            .to_dest(&current_exe)?;

        display.print_success("Successfully updated sha from URL!");
        std::process::exit(0);
    }
}

pub struct VersionCommand;
impl VersionCommand {
    pub fn exec(
        component: Option<String>,
        dry_run: bool,
        use_cases: &WorkspaceUseCases,
    ) -> Result<()> {
        let root = use_cases.find_root()?;

        if let Some(comp) = component {
            let bump = match comp.as_str() {
                "major" => VersionBump::Major,
                "minor" => VersionBump::Minor,
                "patch" => VersionBump::Patch,
                "auto" => VersionBump::Auto,
                _ => return Err(anyhow::anyhow!("Invalid version component: {}", comp)),
            };

            use_cases.bump_version(&root, bump, dry_run)?;
        } else {
            let version = use_cases.get_version(&root)?;
            println!("{}", format!("Current version: {}", version).cyan());
        }

        Ok(())
    }
}

pub struct EnvCommand;
impl EnvCommand {
    pub fn exec(
        action: EnvAction,
        dry_run: bool,
        use_cases: &WorkspaceUseCases,
    ) -> Result<()> {
        let root = use_cases.find_root()?;

        match action {
            EnvAction::Set { key, value } => {
                use_cases.env_set(&root, &key, &value, dry_run)?;
                if !dry_run {
                    println!("{}", format!("Environment variable {} set.", key).green());
                }
            }
            EnvAction::Get { key } => {
                if dry_run {
                    println!("{}", format!("Would get env var: {}", key).yellow());
                    return Ok(());
                }
                match use_cases.env_get(&root, &key)? {
                    Some(value) => {
                        let mut table = Table::new();
                        table.set_header(vec!["Key", "Value"]);
                        table.add_row(vec![key.cyan().to_string(), value.yellow().to_string()]);
                        println!("{table}");
                    }
                    None => {
                        return Err(anyhow::anyhow!("Environment variable {} not found", key));
                    }
                }
            }
            EnvAction::List => {
                if dry_run {
                    println!("{}", "Would list all env vars".yellow());
                    return Ok(());
                }
                let envs = use_cases.env_list(&root)?;
                if envs.is_empty() {
                    println!("{}", "No environment variables found.".yellow());
                } else {
                    let mut table = Table::new();
                    table.set_header(vec!["Key", "Value"]);
                    for env_var in envs {
                        table.add_row(vec![
                            env_var.key.cyan().to_string(),
                            env_var.value.yellow().to_string(),
                        ]);
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
    pub fn exec(
        url: Option<String>,
        use_cases: &WorkspaceUseCases,
        command: &dyn CommandPort,
    ) -> Result<()> {
        let root = use_cases.find_root()?;

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
            std::fs::create_dir_all(&shared_dir)?;
            let dest = shared_dir.join("remote_schema.json");
            std::fs::write(&dest, bytes)?;
            println!("{}", format!("Saved remote schema to {:?}", dest).green());
        }

        println!(
            "{}",
            format!("Syncing APIs in workspace: {:?}", root).cyan()
        );

        let mut table = Table::new();
        table.set_header(vec!["Module", "API Sync Status"]);

        let (success, _, _) = command.run_command("just", &["sync-api"], Some(&root))?;
        if success {
            table.add_row(vec!["Root".cyan().to_string(), "Success".green().to_string()]);
        } else {
            table.add_row(vec!["Root".cyan().to_string(), "Failed".red().to_string()]);
        }

        let server_dir = root.join("web/server");
        let client_dir = root.join("web/client");

        if server_dir.exists() && client_dir.exists() {
            println!(
                "{}",
                "Coordinating types between web/server and web/client...".cyan()
            );

            let (success, _, _) = command.run_command("just", &["sync-api"], Some(&root.join("web")))?;
            if success {
                table.add_row(vec![
                    "Web (Full Stack)".cyan().to_string(),
                    "Success".green().to_string(),
                ]);
            } else {
                table.add_row(vec![
                    "Web (Full Stack)".cyan().to_string(),
                    "No 'just sync-api' found or failed".yellow().to_string(),
                ]);
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
    pub fn exec(
        use_cases: &WorkspaceUseCases,
        command: &dyn CommandPort,
    ) -> Result<()> {
        println!(
            "{}",
            "Checking and installing system-wide dependencies...".cyan()
        );

        Self::check_tool("just", "https://just.systems/man/en/chapter_4.html", command)?;
        Self::check_tool("git", "https://git-scm.com/downloads", command)?;
        Self::check_tool("gh", "https://cli.github.com/", command)?;

        Self::install_bun_if_missing(command)?;
        Self::install_uv_if_missing(command)?;
        Self::install_angular_cli_if_missing(command)?;

        println!("{}", "System-wide tools are present.".green());

        let root = match use_cases.find_root() {
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

        let workspace = use_cases.load_workspace(&root)?;

        for feature in &workspace.features {
            let feature_path = match feature.kind {
                FeatureKind::WebFrontend | FeatureKind::WebBackend => "web",
                FeatureKind::LandingPage => "landing",
                FeatureKind::MobileApp => {
                    Self::check_tool("flutter", "https://docs.flutter.dev/get-started/install", command)?;
                    "mobile"
                }
                FeatureKind::Research => {
                    Self::check_tool("pdflatex", "https://www.latex-project.org/get/", command)?;
                    "research"
                }
                FeatureKind::MachineLearning => "ml",
                FeatureKind::HardwareArduino | FeatureKind::HardwareMicroPython | FeatureKind::HardwareEmbeddedRust => {
                    Self::check_tool("pio", "https://docs.platformio.org/en/latest/core/installation.html", command)?;
                    "hardware"
                }
                FeatureKind::Custom => feature.name.as_str(),
            };

            let dir = root.join(feature_path);
            if dir.exists() && dir.join("justfile").exists() {
                println!("{}", format!("Installing dependencies for {}...", feature.name).cyan());
                let (success, _, _) = command.run_command("just", &["deps"], Some(&dir))?;
                if success {
                    println!("{}", format!("Dependencies for {} installed successfully.", feature.name).green());
                } else {
                    println!("{}", format!("Failed to install dependencies for {}.", feature.name).red());
                }
            }
        }

        Ok(())
    }

    fn check_tool(name: &str, install_url: &str, command: &dyn CommandPort) -> Result<()> {
        if command.command_exists(name) {
            println!("{}", format!("✓ {} is installed.", name).green());
            Ok(())
        } else {
            println!("{}", format!("✗ {} is missing.", name).red());
            println!("{}", format!("  Please install it from: {}", install_url).yellow());
            Err(anyhow::anyhow!("Missing required tool: {}", name))
        }
    }

    fn install_bun_if_missing(command: &dyn CommandPort) -> Result<()> {
        if command.command_exists("bun") {
            println!("{}", "✓ Bun is already installed.".green());
        } else {
            println!("{}", "Installing Bun...".yellow());
            if cfg!(windows) {
                command.run_command("powershell", &["-c", "irm https://bun.sh/install.ps1 | iex"], None)?;
            } else {
                command.run_command("sh", &["-c", "curl -fsSL https://bun.sh/install | bash"], None)?;
            }
        }
        Ok(())
    }

    fn install_uv_if_missing(command: &dyn CommandPort) -> Result<()> {
        if command.command_exists("uv") {
            println!("{}", "✓ UV is already installed.".green());
        } else {
            println!("{}", "Installing UV...".yellow());
            if cfg!(windows) {
                command.run_command("powershell", &["-c", "irm https://astral.sh/uv/install.ps1 | iex"], None)?;
            } else {
                command.run_command("sh", &["-c", "curl -LsSf https://astral.sh/uv/install.sh | sh"], None)?;
            }
        }
        Ok(())
    }

    fn install_angular_cli_if_missing(command: &dyn CommandPort) -> Result<()> {
        if command.command_exists("ng") {
            println!("{}", "✓ Angular CLI is already installed.".green());
        } else if command.command_exists("npm") {
            println!("{}", "Installing Angular CLI via npm...".yellow());
            command.run_command("npm", &["install", "-g", "@angular/cli"], None)?;
        } else {
            println!("{}", "npm not found. Skipping Angular CLI installation.".yellow());
        }
        Ok(())
    }
}

pub struct PulseCommand;
impl PulseCommand {
    pub fn exec(use_cases: &WorkspaceUseCases) -> Result<()> {
        let root = use_cases.find_root()?;
        println!("{}", format!("Checking workspace pulse: {:?}", root).cyan());

        let health = use_cases.get_module_health(&root)?;

        let mut table = Table::new();
        table.set_header(vec!["Module", "Status", "Last Heartbeat / Details"]);

        for module in health {
            let status_str = match module.status {
                HealthStatus::Active => "ACTIVE".green(),
                HealthStatus::Idle => "IDLE".yellow(),
                HealthStatus::Running => "RUNNING?".cyan(),
                HealthStatus::Complete => "COMPLETE".green(),
                HealthStatus::Pending => "PENDING".yellow(),
                HealthStatus::Ready => "READY".green(),
                HealthStatus::Unknown => "UNKNOWN".white(),
            };

            table.add_row(vec![
                module.name.cyan().to_string(),
                status_str.to_string(),
                module.details.white().to_string(),
            ]);
        }

        println!("{table}");

        Ok(())
    }
}

pub struct RegistryCommand;
impl RegistryCommand {
    pub fn exec(action: RegistryAction, use_cases: &WorkspaceUseCases) -> Result<()> {
        let root = use_cases.find_root()?;

        match action {
            RegistryAction::Pin { model, weight_path } => {
                let registry_dir = root.join("ml/model_registry").join(&model);
                std::fs::create_dir_all(&registry_dir)?;

                let git_hash = std::process::Command::new("git")
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

                std::fs::write(
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

                for entry in std::fs::read_dir(registry_dir)? {
                    let entry = entry?;
                    if entry.path().is_dir() {
                        let metadata_path = entry.path().join("metadata.json");
                        if metadata_path.exists() {
                            let metadata: serde_json::Value =
                                serde_json::from_str(&std::fs::read_to_string(metadata_path)?)?;
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
    pub fn exec(feature: Option<String>, std_flag: bool, command: &dyn CommandPort) -> Result<()> {
        if std_flag {
            println!(
                "{}",
                "Opening rustup standard library documentation...".cyan()
            );
            command.run_command("rustup", &["doc", "--std"], None)?;
            return Ok(());
        }

        if let Some(f) = feature {
            let root_dir = std::env::current_dir()?;
            let feature_dir = root_dir.join(&f);
            if feature_dir.exists() {
                println!(
                    "{}",
                    format!("Opening documentation for feature: {}", f).cyan()
                );
                command.run_command("just", &["doc"], Some(&feature_dir))?;
            } else {
                return Err(anyhow::anyhow!("Feature {} not found", f));
            }
        } else {
            println!("{}", "Opening rustup documentation...".cyan());
            command.run_command("rustup", &["doc"], None)?;
        }

        Ok(())
    }
}

pub struct IssueCommand;
impl IssueCommand {
    pub fn exec(
        action: IssueAction,
        git: &dyn GitPort,
        display: &dyn DisplayPort,
    ) -> Result<()> {
        match action {
            IssueAction::Start { id, description } => {
                let desc = match description {
                    Some(d) => d,
                    None => {
                        display.print_info("Fetching issue title from GitHub...");
                        let output = std::process::Command::new("gh")
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
                display.print_info(&format!("Starting issue {} on branch {}...", id, branch_name));

                git.create_branch(&branch_name)?;

                display.print_success(&format!("Successfully switched to branch {}.", branch_name));
            }
            IssueAction::Status => {
                let branch = git.current_branch()?;

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
                let branch = git.current_branch()?;

                if !branch.starts_with("issue-") {
                    return Err(anyhow::anyhow!(
                        "Not on an IDD issue branch. Current branch: {}",
                        branch
                    ));
                }

                display.print_info(&format!("Finishing issue on branch {}...", branch));

                display.print_info("Pushing branch to origin...");
                git.push_branch(&branch)?;

                display.print_info("Creating Pull Request...");
                let status = std::process::Command::new("gh")
                    .args(["pr", "create", "--fill"])
                    .status()?;

                if status.success() {
                    display.print_success("Issue finished and PR created successfully!");
                } else {
                    display.print_warning("PR creation failed. You might need to do it manually.");
                }
            }
        }
        Ok(())
    }
}
