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
            println!("Adding feature: {}", feature);
        }
        Commands::Version { component } => {
            println!("Updating version: {:?}", component);
        }
        Commands::Env { action } => {
            commands::env::exec(action)?;
        }
        Commands::Run { feature } => {
            println!("Running feature: {}", feature);
        }
        Commands::Build { feature } => {
            println!("Building feature: {}", feature);
        }
        Commands::Test { feature } => {
            println!("Testing feature: {}", feature);
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
