pub mod add;
pub mod deps;
pub mod docs;
pub mod env;
pub mod issue;
pub mod new;
pub mod pulse;
pub mod registry;
pub mod restore;
pub mod sync_api;
pub mod version;

use clap::{Parser, Subcommand};

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
        feature: String,
    },
    /// Ensures all enabled features have their required files/folders
    Restore,
    /// Updates Semantic Versioning for the project
    Version {
        /// Version component to increment
        #[arg(value_parser = ["major", "minor", "patch"])]
        component: Option<String>,
    },
    /// Manages project-wide environment variables
    Env {
        #[command(subcommand)]
        action: EnvAction,
    },
    /// Automatically generates clients from Zod/Pydantic definitions
    SyncApi,
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
    Start { id: u64, description: String },
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
