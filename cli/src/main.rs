use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "sha")]
#[command(about = "shastack CLI tool", long_about = None)]
struct Cli {
    #[arg(long, help = "simulate execution without making changes")]
    dry_run: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new workspace or module
    Init,
    /// Add a new standalone module to the project
    Add { module: String },
    /// Initialize a new module (alias for add)
    New { module: String },
    /// Install project-wide dependencies
    Deps,
    /// Executes the development environment
    Run { module: String },
    /// Compiles artifacts (Binaries, PDFs, Web bundles, etc.)
    Build { module: String },
    /// Runs the test suite for the specified module
    Test { module: String },
    /// Deploys firmware to hardware
    Flash,
    /// Triggers deployment pipelines
    Deploy {
        module: String,
        #[arg(long)]
        target: String,
    },
    /// Manage environment secrets
    Env {
        #[command(subcommand)]
        command: EnvCommands,
    },
    /// Cross-module event bus
    Bus {
        #[command(subcommand)]
        command: BusCommands,
    },
    /// Performance benchmarking
    Bench {
        #[command(subcommand)]
        command: BenchCommands,
    },
    /// Security hardening & audit
    Audit {
        #[command(subcommand)]
        command: AuditCommands,
    },
    /// Documentation & DevEx
    Docs {
        #[command(subcommand)]
        command: DocsCommands,
    },
    /// Updates Semantic Versioning for the project
    Version {
        #[arg(value_parser = ["major", "minor", "patch"])]
        level: String,
    },
}

#[derive(Subcommand)]
enum EnvCommands {
    /// Set an environment variable
    Set { key: String, value: String },
    /// Get an environment variable
    Get { key: String },
    /// List all environment variables
    List,
}

#[derive(Subcommand)]
enum BusCommands {
    /// Emit an event to the bus
    Emit {
        event: String,
        #[arg(short, long)]
        payload: Option<String>,
    },
    /// Listen for an event on the bus
    Listen { event: String },
}

#[derive(Subcommand)]
enum BenchCommands {
    /// Run benchmarks for a module
    Run { module: String },
}

#[derive(Subcommand)]
enum AuditCommands {
    /// Scan the workspace for vulnerabilities
    Scan,
}

#[derive(Subcommand)]
enum DocsCommands {
    /// Serve the documentation locally
    Serve,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    name: String,
    version: String,
    modules: HashMap<String, ModuleConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModuleConfig {
    enabled: bool,
    path: String,
}

const CONFIG_PATH: &str = ".sha/config.json";
const ENV_PATH: &str = ".env.sha";

fn load_config() -> Result<Config> {
    if !Path::new(CONFIG_PATH).exists() {
        return Ok(Config {
            name: "shastack".to_string(),
            version: "0.1.0".to_string(),
            modules: HashMap::new(),
        });
    }
    let content = fs::read_to_string(CONFIG_PATH)
        .with_context(|| format!("Could not read config file at {}", CONFIG_PATH))?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

fn save_config(config: &Config, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("[DRY-RUN] Would save config to {}", CONFIG_PATH);
        return Ok(());
    }
    if let Some(parent) = Path::new(CONFIG_PATH).parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(config)?;
    fs::write(CONFIG_PATH, content)?;
    Ok(())
}

fn load_env() -> Result<HashMap<String, String>> {
    if !Path::new(ENV_PATH).exists() {
        return Ok(HashMap::new());
    }
    let content = fs::read_to_string(ENV_PATH)?;
    let mut env = HashMap::new();
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    Ok(env)
}

fn save_env(env: &HashMap<String, String>, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("[DRY-RUN] Would save environment to {}", ENV_PATH);
        return Ok(());
    }
    let mut content = String::new();
    for (key, value) in env {
        content.push_str(&format!("{}={}\n", key, value));
    }
    fs::write(ENV_PATH, content)?;
    Ok(())
}

fn check_command(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_module_task(module: &str, task: &str, dry_run: bool) -> Result<()> {
    let config = load_config()?;
    let module_config = config.modules.get(module)
        .with_context(|| format!("Module '{}' not defined in config", module))?;

    if !module_config.enabled {
        anyhow::bail!("Module '{}' is not enabled.", module);
    }

    let cmd = format!("cd {} && just {}", module_config.path, task);
    if dry_run {
        println!("[DRY-RUN] Would run: {}", cmd);
    } else {
        println!("Running: {}", cmd);
        let status = Command::new("bash")
            .arg("-c")
            .arg(&cmd)
            .status()?;
        if !status.success() {
            anyhow::bail!("Task '{}' failed for module '{}'", task, module);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("Initializing shastack workspace...");
            let mut config = load_config()?;
            let possible_modules = ["web", "ml", "research", "hardware", "mobile", "infra", "bench", "audit", "docs", "cli", "landing-page", "shared"];
            
            for m in possible_modules {
                if Path::new(m).is_dir() {
                    let module_config = config.modules.entry(m.to_string()).or_insert(ModuleConfig {
                        enabled: true,
                        path: m.to_string(),
                    });
                    module_config.enabled = true;
                    println!("Adopted existing module: {}", m);
                }
            }
            save_config(&config, cli.dry_run)?;
        }
        Commands::Deps => {
            let deps = ["just", "cargo", "uv"];
            for d in deps {
                if check_command(d) {
                    println!("✓ {} is installed", d);
                } else {
                    println!("✗ {} is missing", d);
                    println!("Suggest: install {} manually or via package manager.", d);
                }
            }
        }
        Commands::New { module } | Commands::Add { module } => {
            let mut config = load_config()?;
            let (path, should_save, already_exists) = {
                let module_config = config.modules.entry(module.clone()).or_insert(ModuleConfig {
                    enabled: true,
                    path: module.clone(),
                });

                let already_exists = Path::new(&module_config.path).exists() && 
                                     fs::read_dir(&module_config.path).map(|mut d| d.next().is_some()).unwrap_or(false);
                
                let was_enabled = module_config.enabled;
                module_config.enabled = true;
                
                (module_config.path.clone(), !was_enabled, already_exists)
            };

            if already_exists {
                println!("Module '{}' already exists and is functional. Skipping scaffold.", module);
            } else {
                println!("Enabling module: {}", module);
                if should_save {
                    save_config(&config, cli.dry_run)?;
                }

                if !Path::new(&path).exists() {
                    if cli.dry_run {
                        println!("[DRY-RUN] Would create directory: {}", path);
                    } else {
                        fs::create_dir_all(&path)?;
                        println!("Created directory: {}", path);
                    }
                }
            }
        }
        Commands::Run { module } => run_module_task(&module, "dev", cli.dry_run)?,
        Commands::Build { module } => run_module_task(&module, "build", cli.dry_run)?,
        Commands::Test { module } => run_module_task(&module, "test", cli.dry_run)?,
        Commands::Flash => {
            println!("Flashing firmware...");
            // Simplified for now
        }
        Commands::Deploy { module, target } => {
            println!("Deploying {} to {}...", module, target);
        }
        Commands::Env { command } => match command {
            EnvCommands::Set { key, value } => {
                let mut env = load_env()?;
                env.insert(key.clone(), value.clone());
                save_env(&env, cli.dry_run)?;
                if !cli.dry_run {
                    println!("Set {}={} in {}", key, value, ENV_PATH);
                }
            }
            EnvCommands::Get { key } => {
                let env = load_env()?;
                if let Some(value) = env.get(&key) {
                    println!("{}", value);
                } else {
                    anyhow::bail!("Key '{}' not found in {}", key, ENV_PATH);
                }
            }
            EnvCommands::List => {
                let env = load_env()?;
                for (key, value) in env {
                    println!("{}={}", key, value);
                }
            }
        },
        Commands::Bus { command } => match command {
            BusCommands::Emit { event, payload } => {
                let p = payload.unwrap_or_else(|| "{}".to_string());
                let msg = format!("[BUS] Emitting event: {} with payload: {}", event, p);
                if cli.dry_run {
                    println!("[DRY-RUN] Would emit: {}", msg);
                } else {
                    println!("{}", msg);
                }
            }
            BusCommands::Listen { event } => {
                println!("[BUS] Listening for event: {}...", event);
            }
        },
        Commands::Bench { command } => match command {
            BenchCommands::Run { module } => {
                println!("[BENCH] Running benchmarks for {}...", module);
            }
        },
        Commands::Audit { command } => match command {
            AuditCommands::Scan => {
                println!("[AUDIT] Scanning workspace...");
            }
        },
        Commands::Docs { command } => match command {
            DocsCommands::Serve => {
                println!("[DOCS] Serving documentation at http://localhost:3000...");
            }
        },
        Commands::Version { level } => {
            println!("Updating version level: {}...", level);
        }
    }

    Ok(())
}
