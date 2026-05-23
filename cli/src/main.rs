mod commands;
pub mod workspace;

use anyhow::Result;
use clap::Parser;
use commands::{
    AddCommand, Cli, Commands, DepsCommand, DocsCommand, EnvCommand, IssueCommand, NewCommand,
    PulseCommand, RegistryCommand, RestoreCommand, SyncApiCommand, UpdateCommand,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check for updates on every run, unless it's the upgrade command itself
    if !matches!(cli.command, Commands::Upgrade { .. }) {
        let _ = UpdateCommand::check_for_updates();
    }

    match cli.command {
        Commands::New { name } => {
            NewCommand::exec(name)?;
        }
        Commands::Add { feature } => {
            AddCommand::exec(feature)?;
        }
        Commands::Restore => {
            RestoreCommand::exec()?;
        }
        Commands::Upgrade { url } => {
            UpdateCommand::exec(url)?;
        }
        Commands::Version { component } => {
            commands::VersionCommand::exec(component)?;
        }
        Commands::Env { action } => {
            EnvCommand::exec(action)?;
        }
        Commands::SyncApi { url } => {
            SyncApiCommand::exec(url)?;
        }
        Commands::Deps => {
            DepsCommand::exec()?;
        }
        Commands::Pulse => {
            PulseCommand::exec()?;
        }
        Commands::Registry { action } => {
            RegistryCommand::exec(action)?;
        }
        Commands::Docs { feature, std } => {
            DocsCommand::exec(feature, std)?;
        }
        Commands::Issue { action } => {
            IssueCommand::exec(action)?;
        }
    }

    Ok(())
}
