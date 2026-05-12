pub mod commands;
pub mod workspace;

use anyhow::Result;
use clap::Parser;
use commands::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check for updates on every run, unless it's the upgrade command itself
    if !matches!(cli.command, Commands::Upgrade) {
        let _ = commands::update::check_for_updates();
    }

    match cli.command {
        Commands::New { name } => {
            commands::new::exec(name)?;
        }
        Commands::Add { feature } => {
            commands::add::exec(feature)?;
        }
        Commands::Restore => {
            commands::restore::exec()?;
        }
        Commands::Upgrade => {
            commands::update::check_for_updates()?;
        }
        Commands::Version { component } => {
            commands::version::exec(component)?;
        }
        Commands::Env { action } => {
            commands::env::exec(action)?;
        }
        Commands::SyncApi => {
            commands::sync_api::exec()?;
        }
        Commands::Deps => {
            commands::deps::exec()?;
        }
        Commands::Pulse => {
            commands::pulse::exec()?;
        }
        Commands::Registry { action } => {
            commands::registry::exec(action)?;
        }
        Commands::Docs { feature, std } => {
            commands::docs::exec(feature, std)?;
        }
        Commands::Issue { action } => {
            commands::issue::exec(action)?;
        }
    }

    Ok(())
}
