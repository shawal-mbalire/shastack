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
        Commands::Version { component } => {
            commands::version::exec(component)?;
        }
        Commands::Env { action } => {
            commands::env::exec(action)?;
        }
        Commands::Run { feature } => {
            commands::wrappers::run(feature)?;
        }
        Commands::Build { feature } => {
            commands::wrappers::build(feature)?;
        }
        Commands::Test { feature } => {
            commands::wrappers::test(feature)?;
        }
        Commands::Flash => {
            commands::wrappers::flash()?;
        }
        Commands::Deploy { feature, target } => {
            commands::wrappers::deploy(feature, target)?;
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
