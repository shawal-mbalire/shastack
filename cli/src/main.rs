mod domain;
mod adapters;
mod commands;

use anyhow::Result;
use clap::Parser;
use commands::{
    Cli, Commands,
    NewCommand, AddCommand, RestoreCommand, UpdateCommand, VersionCommand,
    EnvCommand, SyncApiCommand, DepsCommand, PulseCommand, RegistryCommand,
    DocsCommand, IssueCommand,
};
use adapters::fs::RealFileSystem;
use adapters::git::RealGit;
use adapters::scaffold::RealScaffolder;
use adapters::env::RealEnv;
use adapters::display::RealDisplay;
use adapters::http::RealHttp;
use adapters::prompt::RealPrompt;
use adapters::command::RealCommand;
use domain::use_cases::WorkspaceUseCases;
use domain::ports::DisplayPort;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Create adapters (composition root)
    let fs = RealFileSystem;
    let git = RealGit;
    let scaffold = RealScaffolder;
    let env = RealEnv;
    let display = RealDisplay;
    let http = RealHttp;
    let prompt = RealPrompt;
    let command = RealCommand;

    // Create use case with injected adapters
    let use_cases = WorkspaceUseCases::new(&fs, &git, &scaffold, &env, &display);

    if cli.dry_run {
        display.print_dry_run("DRY RUN MODE - No changes will be made\n");
    }

    // Check for updates on every run
    if !matches!(cli.command, Commands::Upgrade { .. }) {
        let _ = UpdateCommand::check_for_updates(&http, &display);
    }

    match cli.command {
        Commands::New { name } => {
            NewCommand::exec(name, cli.dry_run, &use_cases, &prompt)?;
        }
        Commands::Add { feature } => {
            AddCommand::exec(feature, cli.dry_run, &use_cases, &prompt)?;
        }
        Commands::Restore => {
            RestoreCommand::exec(cli.dry_run, &use_cases)?;
        }
        Commands::Upgrade { url } => {
            UpdateCommand::exec(url, &http, &display)?;
        }
        Commands::Version { component } => {
            VersionCommand::exec(component, cli.dry_run, &use_cases)?;
        }
        Commands::Env { action } => {
            EnvCommand::exec(action, cli.dry_run, &use_cases)?;
        }
        Commands::SyncApi { url } => {
            SyncApiCommand::exec(url, &use_cases, &command)?;
        }
        Commands::Deps => {
            DepsCommand::exec(&use_cases, &command)?;
        }
        Commands::Pulse => {
            PulseCommand::exec(&use_cases)?;
        }
        Commands::Registry { action } => {
            RegistryCommand::exec(action, &use_cases)?;
        }
        Commands::Docs { feature, std } => {
            DocsCommand::exec(feature, std, &command)?;
        }
        Commands::Issue { action } => {
            IssueCommand::exec(action, &git, &display)?;
        }
    }

    Ok(())
}
