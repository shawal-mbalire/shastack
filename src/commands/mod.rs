pub mod add;
pub mod env;
pub mod new;
pub mod version;
pub mod wrappers;

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
    /// Executes the development environment
    Run {
        /// Feature to run
        feature: String,
    },
    /// Compiles artifacts
    Build {
        /// Feature to build
        feature: String,
    },
    /// Runs the test suite
    Test {
        /// Feature to test
        feature: String,
    },
    /// Deploys firmware to hardware
    Flash,
    /// Triggers deployment pipelines
    Deploy {
        /// Feature to deploy
        feature: String,
        /// Target platform
        #[arg(long)]
        target: String,
    },
}

#[derive(Subcommand)]
pub enum EnvAction {
    /// Set an environment variable
    Set { key: String, value: String },
    /// Get an environment variable
    Get { key: String },
}
