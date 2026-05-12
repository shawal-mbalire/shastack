pub mod scaffold;

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

const WEB_CI_TEMPLATE: &str = r#"name: Web CI

on:
  push:
    paths:
      - 'web/**'
  pull_request:
    paths:
      - 'web/**'

jobs:
  test:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: web
    steps:
      - uses: actions/checkout@v4
      - uses: extractions/setup-just@v2
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - uses: oven-sh/setup-bun@v2
      - name: Install dependencies and run tests
        run: |
          just deps
          just test
"#;

const MOBILE_CI_TEMPLATE: &str = r#"name: Mobile CI

on:
  push:
    paths:
      - 'mobile/**'
  pull_request:
    paths:
      - 'mobile/**'

jobs:
  test:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: mobile
    steps:
      - uses: actions/checkout@v4
      - uses: extractions/setup-just@v2
      - uses: subosito/flutter-action@v2
        with:
          channel: stable
      - name: Install dependencies and run tests
        run: |
          just deps
          just test
"#;

const RESEARCH_CI_TEMPLATE: &str = r#"name: Research CI

on:
  push:
    paths:
      - 'research/**'
  pull_request:
    paths:
      - 'research/**'

jobs:
  build:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: research
    steps:
      - uses: actions/checkout@v4
      - uses: extractions/setup-just@v2
      - name: Install LaTeX toolchain
        run: |
          sudo apt-get update
          sudo apt-get install -y texlive-latex-extra biber
      - name: Build paper
        run: just build
"#;

const HARDWARE_CI_TEMPLATE: &str = r#"name: Hardware CI

on:
  push:
    paths:
      - 'hardware/**'
  pull_request:
    paths:
      - 'hardware/**'

jobs:
  build:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: hardware
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'
      - name: Install PlatformIO
        run: pip install platformio
      - name: Compile firmware
        run: pio run
"#;

const LANDING_CI_TEMPLATE: &str = r#"name: Landing CI

on:
  push:
    paths:
      - 'landing/**'
  pull_request:
    paths:
      - 'landing/**'

jobs:
  test:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: landing
    steps:
      - uses: actions/checkout@v4
      - uses: extractions/setup-just@v2
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - name: Install dependencies and run tests
        run: |
          just deps
          just test
"#;

const PYTHON_CI_TEMPLATE: &str = r#"name: Python CI

on:
  push:
    paths:
      - 'web/server/**'
      - 'ml/**'
  pull_request:
    paths:
      - 'web/server/**'
      - 'ml/**'

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: extractions/setup-just@v2
      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'
      - uses: astral-sh/setup-uv@v5
      - name: Install dependencies and run tests
        run: |
          just deps
          just test
"#;

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
    Err(anyhow::anyhow!(
        "Not a shastack workspace (no .sha/config.json found)"
    ))
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

pub fn create_feature_dir(root: &Path, feature: &str) -> Result<()> {
    let feature_path = match feature {
        "web" | "Web Frontend (Angular)" | "Web Backend (Flask)" => {
            fs::create_dir_all(root.join("web/client"))?;
            fs::create_dir_all(root.join("web/server"))?;
            if feature == "Web Frontend (Angular)" {
                scaffold::scaffold_web_client(root)?;
            } else if feature == "Web Backend (Flask)" {
                scaffold::scaffold_flask(root)?;
            } else {
                scaffold::scaffold_web(root)?;
            }
            "web"
        }
        "landing" | "Landing Page (Angular)" => {
            fs::create_dir_all(root.join("landing"))?;
            scaffold::scaffold_landing(root)?;
            "landing"
        }
        "mobile" | "Mobile App (Flutter)" => {
            fs::create_dir_all(root.join("mobile/app"))?;
            scaffold::scaffold_mobile(root)?;
            "mobile"
        }
        "research" | "Research (LaTeX)" => {
            fs::create_dir_all(root.join("research/src"))?;
            fs::create_dir_all(root.join("research/artifacts"))?;
            scaffold::scaffold_research(root)?;
            "research"
        }
        "ml" | "ML (Python/Notebooks)" => {
            fs::create_dir_all(root.join("ml/notebooks"))?;
            fs::create_dir_all(root.join("ml/src"))?;
            fs::create_dir_all(root.join("ml/model_registry"))?;
            scaffold::scaffold_ml(root)?;
            "ml"
        }
        "hardware" | "Hardware (Arduino/C++)" => {
            scaffold::scaffold_arduino(root)?;
            "hardware"
        }
        "Hardware (MicroPython/uv)" => {
            scaffold::scaffold_micropython(root)?;
            "hardware"
        }
        "Hardware (Embedded Rust)" => {
            scaffold::scaffold_rust_embedded(root)?;
            "hardware"
        }
        "Hardware (Firmware)" => {
            fs::create_dir_all(root.join("hardware/src"))?;
            scaffold::scaffold_hardware(root)?;
            "hardware"
        }
        _ => {
            fs::create_dir_all(root.join(feature))?;
            feature
        }
    };

    let ci_dir = root.join(feature_path).join(".github/workflows");
    fs::create_dir_all(&ci_dir)?;

    let ci_content = match feature_path {
        "web" => {
            if feature == "Web Backend (Flask)" {
                PYTHON_CI_TEMPLATE.to_string()
            } else {
                WEB_CI_TEMPLATE.to_string()
            }
        }
        "landing" => LANDING_CI_TEMPLATE.to_string(),
        "mobile" => MOBILE_CI_TEMPLATE.to_string(),
        "research" => RESEARCH_CI_TEMPLATE.to_string(),
        "ml" => PYTHON_CI_TEMPLATE.to_string(),
        "hardware" => HARDWARE_CI_TEMPLATE.to_string(),
        _ => format!(
            "name: {} CI\n\non:\n  push:\n    paths:\n      - '{}/**'\n  pull_request:\n    paths:\n      - '{}/**'\n\njobs:\n  validate:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - name: No module checks configured\n        run: echo 'No CI template configured for {}'\n",
            feature_path, feature_path, feature_path, feature_path
        ),
    };
    fs::write(ci_dir.join("main.yml"), ci_content)?;

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

pub fn get_env(root: &Path, key: &str) -> Result<Option<String>> {
    let env_path = root.join(".env.sha");
    if !env_path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(env_path)?;
    for line in content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return Ok(Some(v.trim().to_string()));
            }
        }
    }
    Ok(None)
}

pub fn list_envs(root: &Path) -> Result<Vec<(String, String)>> {
    let env_path = root.join(".env.sha");
    if !env_path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(env_path)?;
    let mut envs = Vec::new();
    for line in content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            envs.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    Ok(envs)
}

pub fn set_env(root: &Path, key: &str, value: &str) -> Result<()> {
    let env_path = root.join(".env.sha");
    let mut lines: Vec<String> = if env_path.exists() {
        fs::read_to_string(&env_path)?
            .lines()
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    };

    let mut found = false;
    for line in lines.iter_mut() {
        if let Some((k, _)) = line.split_once('=') {
            if k.trim() == key {
                *line = format!("{}={}", key, value);
                found = true;
                break;
            }
        }
    }

    if !found {
        lines.push(format!("{}={}", key, value));
    }

    fs::write(env_path, lines.join("\n") + "\n")?;
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

# Install project-wide and system-wide dependencies
deps:
    sha deps
    @echo "Installing module dependencies..."

test:
    @echo "Running tests..."
"#;
    fs::write(root.join("justfile"), justfile_content)?;

    // Create root CI coordinator
    let root_ci_dir = root.join(".github/workflows");
    fs::create_dir_all(&root_ci_dir)?;
    let root_ci_content = r#"name: Global CI Coordinator

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run Global Tests
        run: just test
"#;
    fs::write(root_ci_dir.join("main.yml"), root_ci_content)?;

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

        init(
            workspace_name,
            vec!["Web Frontend (Angular)", "Research (LaTeX)"],
        )?;

        assert!(workspace_path.exists());
        assert!(workspace_path.join(".sha/config.json").exists());
        assert!(workspace_path.join("web/client").exists());
        assert!(workspace_path.join("web/client/package.json").exists());
        assert!(workspace_path.join("web/server/src/index.ts").exists());
        assert!(workspace_path.join("research/src").exists());
        assert!(workspace_path.join("research/main.tex").exists());
        assert!(workspace_path.join(".github/workflows/main.yml").exists());
        assert!(
            workspace_path
                .join("web/.github/workflows/main.yml")
                .exists()
        );
        assert!(
            workspace_path
                .join("research/.github/workflows/main.yml")
                .exists()
        );

        let web_ci = fs::read_to_string(workspace_path.join("web/.github/workflows/main.yml"))?;
        let research_ci =
            fs::read_to_string(workspace_path.join("research/.github/workflows/main.yml"))?;
        assert!(web_ci.contains("web/**"));
        assert!(web_ci.contains("just deps"));
        assert!(research_ci.contains("research/**"));
        assert!(research_ci.contains("just build"));

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
        assert!(workspace_path.join("research/main.tex").exists());

        Ok(())
    }

    #[test]
    fn test_feature_specific_scaffolds() -> Result<()> {
        let dir = tempdir()?;
        let workspace_path = dir.path().join("test_feature_scaffolds");
        let workspace_name = workspace_path.to_str().unwrap();

        init(
            workspace_name,
            vec![
                "Mobile App (Flutter)",
                "ML (Python/Notebooks)",
                "Hardware (Firmware)",
            ],
        )?;

        assert!(workspace_path.join("mobile/app/pubspec.yaml").exists());
        assert!(
            workspace_path
                .join("mobile/app/test/widget_test.dart")
                .exists()
        );
        assert!(workspace_path.join("ml/pyproject.toml").exists());
        assert!(workspace_path.join("ml/notebooks/01_eda.ipynb").exists());
        assert!(workspace_path.join("ml/tests/test_smoke.py").exists());
        assert!(workspace_path.join("hardware/platformio.ini").exists());
        assert!(workspace_path.join("hardware/src/main.cpp").exists());

        let ml_ci = fs::read_to_string(workspace_path.join("ml/.github/workflows/main.yml"))?;
        let hardware_ci =
            fs::read_to_string(workspace_path.join("hardware/.github/workflows/main.yml"))?;
        assert!(ml_ci.contains("ml/**"));
        assert!(ml_ci.contains("just test"));
        assert!(hardware_ci.contains("hardware/**"));
        assert!(hardware_ci.contains("pio run"));

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

    #[test]
    fn test_env_management() -> Result<()> {
        let dir = tempdir()?;
        let workspace_path = dir.path().join("test_env_ws");
        let workspace_name = workspace_path.to_str().unwrap();

        init(workspace_name, vec![])?;
        set_env(&workspace_path, "TEST_KEY", "TEST_VALUE")?;
        assert_eq!(
            get_env(&workspace_path, "TEST_KEY")?,
            Some("TEST_VALUE".to_string())
        );

        set_env(&workspace_path, "TEST_KEY", "UPDATED_VALUE")?;
        assert_eq!(
            get_env(&workspace_path, "TEST_KEY")?,
            Some("UPDATED_VALUE".to_string())
        );

        Ok(())
    }
}
