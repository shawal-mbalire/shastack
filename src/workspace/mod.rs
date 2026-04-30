use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    loop {
        if current.join(".sha/config.json").exists() {
            return Ok(current);
        }
        if !current.pop() {
            break;
        }
    }
    Err(anyhow::anyhow!("Not a shastack workspace (no .sha/config.json found)"))
}

pub fn add_feature(root: &Path, feature: &str) -> Result<()> {
    let config_path = root.join(".sha/config.json");
    let mut manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(&config_path)?)?;

    let features = manifest["features"]
        .as_array_mut()
        .ok_or_else(|| anyhow::anyhow!("Invalid config.json"))?;

    if features.iter().any(|f| f == feature) {
        return Err(anyhow::anyhow!("Feature {} already exists", feature));
    }

    features.push(serde_json::json!(feature));

    // Create feature directory
    create_feature_dir(root, feature)?;

    fs::write(config_path, serde_json::to_string_pretty(&manifest)?)?;

    Ok(())
}

fn create_feature_dir(root: &Path, feature: &str) -> Result<()> {
    match feature {
        "web" | "Web Frontend (Angular)" | "Web Backend (Flask)" => {
            fs::create_dir_all(root.join("web/client"))?;
            fs::create_dir_all(root.join("web/server"))?;
        }
        "mobile" | "Mobile App (Flutter)" => {
            fs::create_dir_all(root.join("mobile/app"))?;
        }
        "research" | "Research (LaTeX)" => {
            fs::create_dir_all(root.join("research/src"))?;
        }
        "ml" | "ML (Python/Notebooks)" => {
            fs::create_dir_all(root.join("ml/notebooks"))?;
            fs::create_dir_all(root.join("ml/src"))?;
        }
        "hardware" | "Hardware (Firmware)" => {
            fs::create_dir_all(root.join("hardware/src"))?;
        }
        _ => {
            // Generic feature directory
            fs::create_dir_all(root.join(feature))?;
        }
    }
    Ok(())
}

use semver::Version;

pub fn get_version(root: &Path) -> Result<Version> {
    let config_path = root.join(".sha/config.json");
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    let version_str = manifest["version"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No version found in config.json"))?;
    Ok(Version::parse(version_str)?)
}

pub fn set_version(root: &Path, version: &Version) -> Result<()> {
    let config_path = root.join(".sha/config.json");
    let mut manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(&config_path)?)?;
    manifest["version"] = serde_json::json!(version.to_string());
    fs::write(config_path, serde_json::to_string_pretty(&manifest)?)?;
    Ok(())
}

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
        "version": "0.1.0",
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
        create_feature_dir(root, feature)?;
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

    #[test]
    fn test_add_feature() -> Result<()> {
        let dir = tempdir()?;
        let workspace_path = dir.path().join("test_add_ws");
        let workspace_name = workspace_path.to_str().unwrap();

        init(workspace_name, vec!["Web Frontend (Angular)"])?;
        add_feature(&workspace_path, "Research (LaTeX)")?;

        let config_path = workspace_path.join(".sha/config.json");
        let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
        let features = manifest["features"].as_array().unwrap();

        assert!(features.contains(&serde_json::json!("Web Frontend (Angular)")));
        assert!(features.contains(&serde_json::json!("Research (LaTeX)")));
        assert!(workspace_path.join("research/src").exists());

        Ok(())
    }

    #[test]
    fn test_versioning() -> Result<()> {
        let dir = tempdir()?;
        let workspace_path = dir.path().join("test_version_ws");
        let workspace_name = workspace_path.to_str().unwrap();

        init(workspace_name, vec![])?;
        let mut version = get_version(&workspace_path)?;
        assert_eq!(version, Version::parse("0.1.0")?);

        version.patch += 1;
        set_version(&workspace_path, &version)?;
        assert_eq!(get_version(&workspace_path)?, Version::parse("0.1.1")?);

        Ok(())
    }
}
