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
            println!("Flashing firmware...");
        }
        Commands::Deploy { feature, target } => {
            println!("Deploying feature {} to target {}", feature, target);
        }
    }

    Ok(())
}
