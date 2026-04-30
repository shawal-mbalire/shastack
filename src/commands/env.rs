use anyhow::Result;
use colored::*;
use comfy_table::Table;
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
                let mut table = Table::new();
                table.set_header(vec!["Key", "Value"]);
                table.add_row(vec![key.cyan().to_string(), value.yellow().to_string()]);
                println!("{table}");
            } else {
                return Err(anyhow::anyhow!("Environment variable {} not found", key));
            }
        }
        EnvAction::List => {
            let envs = workspace::list_envs(&root)?;
            if envs.is_empty() {
                println!("{}", "No environment variables found.".yellow());
            } else {
                let mut table = Table::new();
                table.set_header(vec!["Key", "Value"]);
                for (k, v) in envs {
                    table.add_row(vec![k.cyan().to_string(), v.yellow().to_string()]);
                }
                println!("{table}");
            }
        }
    }
    Ok(())
}
