use anyhow::Result;
use colored::*;
use crate::workspace;

pub fn exec(component: Option<String>) -> Result<()> {
    let root = workspace::find_root()?;
    let mut version = workspace::get_version(&root)?;

    if let Some(comp) = component {
        match comp.as_str() {
            "major" => {
                version.major += 1;
                version.minor = 0;
                version.patch = 0;
            }
            "minor" => {
                version.minor += 1;
                version.patch = 0;
            }
            "patch" => {
                version.patch += 1;
            }
            _ => return Err(anyhow::anyhow!("Invalid version component: {}", comp)),
        }
        workspace::set_version(&root, &version)?;
        println!("{}", format!("Updated version to {}", version).green());
    } else {
        println!("{}", format!("Current version: {}", version).cyan());
    }

    Ok(())
}
