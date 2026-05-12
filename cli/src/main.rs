pub mod commands;
pub mod workspace;

use anyhow::Result;
use clap::Parser;
use commands::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

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
