use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn init(name: &str, features: Vec<&str>) -> Result<()> {
    let root = Path::new(name);
    if root.exists() {
        return Err(anyhow::anyhow!("Directory {} already exists", name));
    }

    fs::create_dir_all(root)?;

    // Create .sha directory
    fs::create_dir_all(root.join(".sha"))?;

    // Create feature manifest
    let manifest = serde_json::json!({
        "name": name,
        "features": features,
    });
    fs::write(
        root.join(".sha/config.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;

    // Create root justfile
    let justfile_content = r#"set shell := ["bash", "-uc"]

# --- Global Commands ---

deps:
    @echo "Installing dependencies..."

test:
    @echo "Running tests..."
"#;
    fs::write(root.join("justfile"), justfile_content)?;

    // Create feature directories
    for feature in features {
        match feature {
            "Web Frontend (Angular)" => {
                fs::create_dir_all(root.join("web/client"))?;
            }
            "Web Backend (Flask)" => {
                fs::create_dir_all(root.join("web/server"))?;
            }
            "Mobile App (Flutter)" => {
                fs::create_dir_all(root.join("mobile/app"))?;
            }
            "Research (LaTeX)" => {
                fs::create_dir_all(root.join("research/src"))?;
            }
            "ML (Python/Notebooks)" => {
                fs::create_dir_all(root.join("ml/notebooks"))?;
                fs::create_dir_all(root.join("ml/src"))?;
            }
            "Hardware (Firmware)" => {
                fs::create_dir_all(root.join("hardware/src"))?;
            }
            _ => {}
        }
    }

    // Create shared directory
    fs::create_dir_all(root.join("shared"))?;

    // Create .env.sha
    fs::write(root.join(".env.sha"), "# shastack secrets\n")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_workspace() -> Result<()> {
        let dir = tempdir()?;
        let workspace_path = dir.path().join("test_ws");
        let workspace_name = workspace_path.to_str().unwrap();

        init(workspace_name, vec!["Web Frontend (Angular)", "Research (LaTeX)"])?;

        assert!(workspace_path.exists());
        assert!(workspace_path.join(".sha/config.json").exists());
        assert!(workspace_path.join("web/client").exists());
        assert!(workspace_path.join("research/src").exists());
        assert!(workspace_path.join("justfile").exists());
        assert!(workspace_path.join(".env.sha").exists());

        Ok(())
    }
}
