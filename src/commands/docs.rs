use anyhow::Result;
use std::process::Command;
use crate::workspace;

pub fn exec(feature: Option<String>, std: bool) -> Result<()> {
    if std {
        println!("Opening rustup standard library documentation...");
        Command::new("rustup")
            .args(["doc", "--std"])
            .status()?;
        return Ok(());
    }

    if let Some(f) = feature {
        let root = workspace::find_root()?;
        let feature_dir = root.join(&f);
        if feature_dir.exists() {
            println!("Opening documentation for feature: {}", f);
            // This assumes 'cargo doc' has been run or we use 'just doc'
            Command::new("just")
                .arg("doc")
                .current_dir(feature_dir)
                .status()?;
        } else {
            return Err(anyhow::anyhow!("Feature {} not found", f));
        }
    } else {
        println!("Opening rustup documentation...");
        Command::new("rustup")
            .arg("doc")
            .status()?;
    }

    Ok(())
}
