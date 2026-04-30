use anyhow::Result;
use crate::commands::EnvAction;

pub fn exec(action: EnvAction) -> Result<()> {
    match action {
        EnvAction::Set { key, value } => {
            println!("Setting env: {}={}", key, value);
            // TODO: Persist to .env.sha
        }
        EnvAction::Get { key } => {
            println!("Getting env: {}", key);
            // TODO: Read from .env.sha
        }
    }
    Ok(())
}
