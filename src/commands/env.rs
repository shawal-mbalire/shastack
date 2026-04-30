use anyhow::Result;
use colored::*;
use crate::commands::EnvAction;
use crate::workspace;

pub fn exec(action: EnvAction) -> Result<()> {
    let root = workspace::find_root()?;

    match action {
        EnvAction::Set { key, value } => {
            workspace::set_env(&root, &key, &value)?;
            println!("{}", format!("Environment variable {} set.", key).green());
        }
        EnvAction::Get { key } => {
            if let Some(value) = workspace::get_env(&root, &key)? {
                println!("{}", value.yellow());
            } else {
                return Err(anyhow::anyhow!("Environment variable {} not found", key));
            }
        }
    }
    Ok(())
}
