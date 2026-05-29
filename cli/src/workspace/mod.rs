use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

// --- CI Templates ---

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

// --- Justfile Templates ---

const ROOT_JUSTFILE_TEMPLATE: &str = r#"set shell := ["bash", "-uc"]

# --- Global Commands ---

# Install all dependencies for enabled modules
deps:
    sha deps

# Run tests for all enabled modules
test:
    #!/usr/bin/env bash
    for dir in */; do
        if [ -f "$dir/justfile" ]; then
            echo "Running tests in $dir..."
            just -f "$dir/justfile" test
        fi
    done

# Sync all APIs
sync-api:
    sha sync-api
"#;

const WEB_JUSTFILE_TEMPLATE: &str = r#"set shell := ["bash", "-uc"]

deps:
    cd client && npm install
    cd server && uv sync

test:
    cd client && npm test -- --watch=false --browsers=ChromeHeadless
    cd server && uv run pytest

run:
    @echo "Use 'just run-client' or 'just run-server' for specific modules"

run-client:
    cd client && npm start

run-server:
    cd server && uv run python main.py
"#;

const PYTHON_JUSTFILE_TEMPLATE: &str = r#"set shell := ["bash", "-uc"]

deps:
    uv sync

test:
    uv run pytest

run:
    uv run python main.py
"#;

const ANGULAR_JUSTFILE_TEMPLATE: &str = r#"set shell := ["bash", "-uc"]

deps:
    npm install

test:
    npm test -- --watch=false --browsers=ChromeHeadless

run:
    npm start
"#;

// --- Scaffolding Struct ---

pub struct Scaffolder;

impl Scaffolder {
    pub fn angular(dir: &Path) -> Result<()> {
        let status = Command::new("npx")
            .arg("@angular/cli@18")
            .arg("new")
            .arg("frontend")
            .arg("--directory")
            .arg(dir.to_str().unwrap())
            .arg("--style=scss")
            .arg("--ssr=false")
            .arg("--ai-config=none")
            .arg("--skip-git=true")
            .arg("--defaults=true")
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to scaffold Angular project"));
        }
        Self::write_justfile(dir, ANGULAR_JUSTFILE_TEMPLATE)?;
        Ok(())
    }

    pub fn python(dir: &Path, deps: Vec<&str>) -> Result<()> {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        
        let status = Command::new("uv")
            .arg("init")
            .current_dir(dir)
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to initialize Python project with uv"));
        }

        if !deps.is_empty() {
            let status = Command::new("uv")
                .arg("add")
                .args(deps)
                .current_dir(dir)
                .status()?;
            if !status.success() {
                return Err(anyhow::anyhow!("Failed to add dependencies with uv"));
            }
        }
        Self::write_justfile(dir, PYTHON_JUSTFILE_TEMPLATE)?;
        Ok(())
    }

    pub fn flutter(dir: &Path) -> Result<()> {
        let status = Command::new("flutter")
            .arg("create")
            .arg("--project-name=app")
            .arg(".")
            .current_dir(dir)
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to scaffold Flutter project"));
        }
        Ok(())
    }

    pub fn research(dir: &Path) -> Result<()> {
        let src_dir = dir.join("src");
        fs::create_dir_all(&src_dir)?;
        fs::write(
            src_dir.join("main.tex"),
            r#"\documentclass{article}
\begin{titlepage}
\title{Research Paper}
\author{shastack}
\end{titlepage}
\begin{document}
\maketitle
\section{Introduction}
Hello from shastack!
\end{document}"#,
        )?;
        Self::write_justfile(
            dir,
            r#"set shell := ["bash", "-uc"]
build:
    pdflatex -output-directory=../artifacts src/main.tex
"#,
        )?;
        Ok(())
    }

    pub fn write_justfile(dir: &Path, content: &str) -> Result<()> {
        fs::write(dir.join("justfile"), content)?;
        Ok(())
    }
}

// --- Workspace Logic ---

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
    create_feature_dir(root, feature)?;
    fs::write(config_path, serde_json::to_string_pretty(&manifest)?)?;

    Ok(())
}

pub fn create_feature_dir(root: &Path, feature: &str) -> Result<()> {
    let feature_path = match feature {
        "web" | "Web Frontend (Angular)" | "Web Backend (Flask)" => {
            let web_root = root.join("web");
            fs::create_dir_all(&web_root)?;
            
            if feature == "Web Frontend (Angular)" || feature == "web" {
                Scaffolder::angular(&web_root.join("client"))?;
            }
            
            if feature == "Web Backend (Flask)" || feature == "web" {
                Scaffolder::python(&web_root.join("server"), vec!["flask", "flask-cors", "pydantic", "python-dotenv"])?;
            }
            
            Scaffolder::write_justfile(&web_root, WEB_JUSTFILE_TEMPLATE)?;
            "web"
        }
        "landing" | "Landing Page (Angular)" => {
            Scaffolder::angular(&root.join("landing"))?;
            "landing"
        }
        "mobile" | "Mobile App (Flutter)" => {
            let mobile_root = root.join("mobile/app");
            fs::create_dir_all(&mobile_root)?;
            Scaffolder::flutter(&mobile_root)?;
            "mobile"
        }
        "research" | "Research (LaTeX)" => {
            let research_root = root.join("research");
            fs::create_dir_all(&research_root)?;
            Scaffolder::research(&research_root)?;
            "research"
        }
        "ml" | "ML (Python/Notebooks)" => {
            Scaffolder::python(&root.join("ml"), vec!["polars", "scikit-learn", "numpy", "pydantic", "huggingface-hub", "jupyter", "nbdev"])?;
            "ml"
        }
        "hardware" | "Hardware (Arduino/C++)" | "Hardware (MicroPython/uv)" | "Hardware (Embedded Rust)" => {
            fs::create_dir_all(root.join("hardware"))?;
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

// --- envchain-backed environment management ---

pub fn envchain_namespace(root: &Path) -> Result<String> {
    let config_path = root.join(".sha/config.json");
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    let name = manifest["name"].as_str().unwrap_or("shastack");
    Ok(format!("shastack-{}", name))
}

/// Path to the key index file (key names only, not values)
fn env_keys_path(root: &Path) -> PathBuf {
    root.join(".sha/env-keys.json")
}

/// Load the list of known env key names from the index
fn load_env_keys(root: &Path) -> Result<Vec<String>> {
    let path = env_keys_path(root);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)?;
    let keys: Vec<String> = serde_json::from_str(&content)?;
    Ok(keys)
}

/// Save the list of known env key names to the index
fn save_env_keys(root: &Path, keys: &[String]) -> Result<()> {
    let path = env_keys_path(root);
    let parent = path.parent().unwrap();
    fs::create_dir_all(parent)?;
    fs::write(path, serde_json::to_string_pretty(keys)?)?;
    Ok(())
}

/// Add a key name to the index (duplicate-safe)
fn add_env_key(root: &Path, key: &str) -> Result<()> {
    let mut keys = load_env_keys(root)?;
    if !keys.contains(&key.to_string()) {
        keys.push(key.to_string());
        save_env_keys(root, &keys)?;
    }
    Ok(())
}

/// Remove a key name from the index
#[allow(dead_code)]
fn remove_env_key(root: &Path, key: &str) -> Result<()> {
    let mut keys = load_env_keys(root)?;
    keys.retain(|k| k != key);
    save_env_keys(root, &keys)?;
    Ok(())
}

/// Migrate environment variables from .env.sha to envchain
/// Called automatically on first `sha env` operation if .env.sha exists
pub fn migrate_from_env_sha(root: &Path) -> Result<()> {
    let env_sha_path = root.join(".env.sha");
    if !env_sha_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&env_sha_path)?;
    let has_content = content.lines().any(|line| {
        let t = line.trim();
        !t.is_empty() && !t.starts_with('#') && t.contains('=')
    });

    if !has_content {
        // Empty or comment-only file, just remove it
        let _ = fs::remove_file(&env_sha_path);
        return Ok(());
    }

    println!("{}", "Migrating secrets from .env.sha to envchain...".yellow());

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() || !line.contains('=') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim();
            let value = v.trim();
            if !key.is_empty() {
                set_env(root, key, value)?;
            }
        }
    }

    // Rename old file as backup (user can delete manually)
    let backup = root.join(".env.sha.migrated");
    fs::rename(&env_sha_path, &backup)?;
    println!(
        "{}",
        format!("Migration complete. Old file backed up to {:?}", backup).green()
    );

    Ok(())
}

/// Check if envchain is available on the system
fn check_envchain() -> Result<()> {
    let status = Command::new("envchain")
        .arg("--help")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err(anyhow::anyhow!(
            "envchain is not installed. Install it with: brew install envchain (macOS) or your package manager (Linux)"
        )),
    }
}

/// Get an environment variable from the envchain keychain
pub fn get_env(root: &Path, key: &str) -> Result<Option<String>> {
    check_envchain()?;
    let _ = migrate_from_env_sha(root);

    let namespace = envchain_namespace(root)?;

    let output = Command::new("envchain")
        .args(["--no-require-passphrase", &namespace, "printenv", key])
        .output()?;

    if output.status.success() {
        let value = String::from_utf8(output.stdout)?.trim().to_string();
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

/// List all environment variables stored in envchain for this workspace
pub fn list_envs(root: &Path) -> Result<Vec<(String, String)>> {
    check_envchain()?;
    let _ = migrate_from_env_sha(root);

    let namespace = envchain_namespace(root)?;

    // Read key names from the index, then fetch each value from the keychain
    let keys = load_env_keys(root)?;
    let mut envs = Vec::new();

    for key in &keys {
        let output = Command::new("envchain")
            .args(["--no-require-passphrase", &namespace, "printenv", key])
            .output()?;

        if output.status.success() {
            let value = String::from_utf8(output.stdout)?.trim().to_string();
            envs.push((key.clone(), value));
        }
        // Skip missing keys silently (may have been removed from keychain externally)
    }

    Ok(envs)
}

/// Set an environment variable in the envchain keychain
pub fn set_env(root: &Path, key: &str, value: &str) -> Result<()> {
    check_envchain()?;
    let _ = migrate_from_env_sha(root);

    let namespace = envchain_namespace(root)?;

    let mut child = Command::new("envchain")
        .args(["--set", "--no-require-passphrase", &namespace, key])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to run envchain: {}. Is envchain installed?", e))?;

    // Pipe the value into envchain's stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(value.as_bytes())?;
        // Drop stdin so envchain can finish reading
        drop(stdin);
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // envchain may still succeed with warnings; only fail on hard error
        if stderr.contains("failed") || stderr.contains("error") {
            return Err(anyhow::anyhow!("envchain set failed: {}", stderr.trim()));
        }
    }

    add_env_key(root, key)?;
    Ok(())
}

pub fn init(name: &str, features: Vec<&str>) -> Result<()> {
    let root = if name == "." {
        std::env::current_dir()?
    } else {
        let p = PathBuf::from(name);
        if p.exists() && fs::read_dir(&p)?.next().is_some() {
            return Err(anyhow::anyhow!("Directory {} already exists and is not empty", name));
        }
        fs::create_dir_all(&p)?;
        p
    };

    let workspace_name = if name == "." {
        root.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unnamed-workspace")
            .to_string()
    } else {
        name.to_string()
    };

    fs::create_dir_all(root.join(".sha"))?;

    let manifest = serde_json::json!({
        "name": workspace_name,
        "version": "0.1.0",
        "features": features,
    });
    fs::write(
        root.join(".sha/config.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;

    fs::write(root.join("justfile"), ROOT_JUSTFILE_TEMPLATE)?;

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

    for feature in features {
        create_feature_dir(&root, feature)?;
    }

    fs::create_dir_all(root.join("shared"))?;

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

        // Use features that are fast or manual for testing
        init(
            workspace_name,
            vec!["Research (LaTeX)"],
        )?;

        assert!(workspace_path.exists());
        assert!(workspace_path.join(".sha/config.json").exists());
        assert!(workspace_path.join("research/src").exists());
        assert!(workspace_path.join(".github/workflows/main.yml").exists());
        assert!(
            workspace_path
                .join("research/.github/workflows/main.yml")
                .exists()
        );
        assert!(!workspace_path.join(".env.sha").exists(), ".env.sha should NOT be created; envchain is used instead");

        Ok(())
    }

    #[test]
    fn test_add_feature() -> Result<()> {
        let dir = tempdir()?;
        let workspace_path = dir.path().join("test_add_ws");
        let workspace_name = workspace_path.to_str().unwrap();

        init(workspace_name, vec![])?;
        add_feature(&workspace_path, "Research (LaTeX)")?;

        let config_path = workspace_path.join(".sha/config.json");
        let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
        let features = manifest["features"].as_array().unwrap();

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

    #[test]
    #[ignore = "requires envchain to be installed with a keychain backend (gnome-keyring/macOS Keychain)"]
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
