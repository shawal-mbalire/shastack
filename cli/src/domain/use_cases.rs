use crate::domain::errors::ShaError;
use crate::domain::models::*;
use crate::domain::ports::*;
use semver::Version;
use std::path::{Path, PathBuf};

pub struct WorkspaceUseCases<'a> {
    fs: &'a dyn FileSystemPort,
    git: &'a dyn GitPort,
    scaffold: &'a dyn ScaffoldPort,
    env: &'a dyn EnvPort,
    display: &'a dyn DisplayPort,
}

impl<'a> WorkspaceUseCases<'a> {
    pub fn new(
        fs: &'a dyn FileSystemPort,
        git: &'a dyn GitPort,
        scaffold: &'a dyn ScaffoldPort,
        env: &'a dyn EnvPort,
        display: &'a dyn DisplayPort,
    ) -> Self {
        Self { fs, git, scaffold, env, display }
    }

    pub fn find_root(&self) -> Result<PathBuf, ShaError> {
        let mut current = self.fs.current_dir()?;
        loop {
            let config = current.join(".sha/config.json");
            if self.fs.file_exists(&config) {
                return Ok(current);
            }
            if !current.pop() {
                break;
            }
        }
        Err(ShaError::NotAWorkspace)
    }

    pub fn load_workspace(&self, root: &Path) -> Result<Workspace, ShaError> {
        let config_path = root.join(".sha/config.json");
        let content = self.fs.read_file(&config_path)?;
        let manifest: serde_json::Value = serde_json::from_str(&content)?;

        let name = manifest["name"].as_str().unwrap_or("unnamed").to_string();
        let version_str = manifest["version"].as_str().unwrap_or("0.1.0");
        let version = Version::parse(version_str)
            .map_err(|e| ShaError::VersionParseError(e.to_string()))?;

        let features = manifest["features"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|name| Feature {
                        name: name.to_string(),
                        kind: FeatureKind::from_name(name),
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(Workspace { root: root.to_path_buf(), name, version, features })
    }

    pub fn init_workspace(&self, name: &str, feature_names: Vec<&str>, dry_run: bool) -> Result<(), ShaError> {
        let root = if name == "." {
            self.fs.current_dir()?
        } else {
            let p = PathBuf::from(name);
            if self.fs.dir_exists(&p) && !self.fs.read_dir(&p)?.is_empty() {
                return Err(ShaError::WorkspaceAlreadyExists(name.to_string()));
            }
            self.fs.create_dir(&p)?;
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

        if dry_run {
            self.display.print_dry_run(&format!("Would create workspace at: {}", name));
            self.display.print_dry_run("Would select features interactively");
            self.display.print_dry_run("Operations that would be performed:");
            self.display.print_dry_run(&format!("  1. Create directory: {}/", name));
            self.display.print_dry_run("  2. Create .sha/config.json");
            self.display.print_dry_run("  3. Create root justfile");
            self.display.print_dry_run("  4. Create .github/workflows/main.yml");
            self.display.print_dry_run("  5. Create shared/ directory");
            return Ok(());
        }

        self.fs.create_dir(&root.join(".sha"))?;

        let features_json: Vec<serde_json::Value> = feature_names
            .iter()
            .map(|f| serde_json::json!(f))
            .collect();

        let manifest = serde_json::json!({
            "name": workspace_name,
            "version": "0.1.0",
            "features": features_json,
        });
        self.fs.write_file(
            &root.join(".sha/config.json"),
            &serde_json::to_string_pretty(&manifest)?,
        )?;

        self.scaffold.write_justfile(&root, ROOT_JUSTFILE)?;

        let ci_dir = root.join(".github/workflows");
        self.fs.create_dir(&ci_dir)?;
        self.scaffold.write_ci_workflow(&ci_dir, ROOT_CI)?;

        for feature_name in &feature_names {
            self.create_feature_dir(&root, feature_name)?;
        }

        self.fs.create_dir(&root.join("shared"))?;
        self.display.print_success(&format!("Workspace {} initialized successfully!", workspace_name));
        Ok(())
    }

    pub fn add_feature(&self, root: &Path, feature_name: &str, dry_run: bool) -> Result<(), ShaError> {
        let config_path = root.join(".sha/config.json");
        let content = self.fs.read_file(&config_path)?;
        let mut manifest: serde_json::Value = serde_json::from_str(&content)?;

        let features = manifest["features"]
            .as_array_mut()
            .ok_or_else(|| ShaError::InvalidConfig("features not an array".to_string()))?;

        if features.iter().any(|f| f.as_str() == Some(feature_name)) {
            return Err(ShaError::FeatureAlreadyExists(feature_name.to_string()));
        }

        if dry_run {
            self.display.print_dry_run(&format!("Would add feature '{}' to workspace at {:?}", feature_name, root));
            self.display.print_dry_run("Operations that would be performed:");
            self.display.print_dry_run("  1. Update .sha/config.json (add feature to list)");
            self.display.print_dry_run("  2. Create feature directory structure");
            self.display.print_dry_run("  3. Create feature CI workflow");
            return Ok(());
        }

        features.push(serde_json::json!(feature_name));
        self.create_feature_dir(root, feature_name)?;
        self.fs.write_file(&config_path, &serde_json::to_string_pretty(&manifest)?)?;

        self.display.print_success(&format!("Feature {} added successfully!", feature_name));
        Ok(())
    }

    pub fn create_feature_dir(&self, root: &Path, feature: &str) -> Result<(), ShaError> {
        let kind = FeatureKind::from_name(feature);
        let feature_path = kind.directory_name();

        match kind {
            FeatureKind::WebFrontend | FeatureKind::WebBackend => {
                let web_root = root.join("web");
                self.fs.create_dir(&web_root)?;

                if feature == "Web Frontend (Angular)" || feature == "web" {
                    self.scaffold.scaffold_angular(&web_root.join("client"))?;
                }
                if feature == "Web Backend (Flask)" || feature == "web" {
                    self.scaffold.scaffold_python(
                        &web_root.join("server"),
                        &["flask", "flask-cors", "pydantic", "python-dotenv"],
                    )?;
                }
                self.scaffold.write_justfile(&web_root, WEB_JUSTFILE)?;
            }
            FeatureKind::LandingPage => {
                self.scaffold.scaffold_angular(&root.join("landing"))?;
            }
            FeatureKind::MobileApp => {
                let mobile_root = root.join("mobile/app");
                self.fs.create_dir(&mobile_root)?;
                self.scaffold.scaffold_flutter(&mobile_root)?;
            }
            FeatureKind::Research => {
                let research_root = root.join("research");
                self.fs.create_dir(&research_root)?;
                self.scaffold.scaffold_research(&research_root)?;
            }
            FeatureKind::MachineLearning => {
                self.scaffold.scaffold_python(
                    &root.join("ml"),
                    &["polars", "scikit-learn", "numpy", "pydantic", "huggingface-hub", "jupyter", "nbdev"],
                )?;
            }
            FeatureKind::HardwareArduino | FeatureKind::HardwareMicroPython | FeatureKind::HardwareEmbeddedRust => {
                self.fs.create_dir(&root.join("hardware"))?;
            }
            FeatureKind::Custom => {
                self.fs.create_dir(&root.join(feature))?;
            }
        }

        let ci_dir = root.join(feature_path).join(".github/workflows");
        self.fs.create_dir(&ci_dir)?;

        let ci_content = match kind {
            FeatureKind::WebBackend => PYTHON_CI,
            FeatureKind::LandingPage => LANDING_CI,
            FeatureKind::MobileApp => MOBILE_CI,
            FeatureKind::Research => RESEARCH_CI,
            FeatureKind::MachineLearning => PYTHON_CI,
            FeatureKind::HardwareArduino | FeatureKind::HardwareMicroPython | FeatureKind::HardwareEmbeddedRust => HARDWARE_CI,
            _ => DEFAULT_CI,
        };
        self.scaffold.write_ci_workflow(&ci_dir, ci_content)?;

        Ok(())
    }

    pub fn restore_workspace(&self, root: &Path, dry_run: bool) -> Result<(), ShaError> {
        let workspace = self.load_workspace(root)?;

        if dry_run {
            self.display.print_dry_run("Features that would be restored:");
            for feature in &workspace.features {
                self.display.print_dry_run(&format!("  - {}", feature.name));
            }
            return Ok(());
        }

        for feature in &workspace.features {
            self.display.print_info(&format!("Restoring feature: {}", feature.name));
            match self.create_feature_dir(root, &feature.name) {
                Ok(()) => {}
                Err(ShaError::FeatureAlreadyExists(_)) => {
                    self.create_feature_dir(root, &feature.name)?;
                }
                Err(e) => return Err(e),
            }
        }

        self.display.print_success("Workspace restoration complete.");
        Ok(())
    }

    pub fn get_version(&self, root: &Path) -> Result<Version, ShaError> {
        let workspace = self.load_workspace(root)?;
        Ok(workspace.version)
    }

    pub fn set_version(&self, root: &Path, version: &Version, dry_run: bool) -> Result<(), ShaError> {
        if dry_run {
            let current = self.get_version(root)?;
            self.display.print_dry_run(&format!("Would update version: {} -> {}", current, version));
            return Ok(());
        }

        let config_path = root.join(".sha/config.json");
        let content = self.fs.read_file(&config_path)?;
        let mut manifest: serde_json::Value = serde_json::from_str(&content)?;
        manifest["version"] = serde_json::json!(version.to_string());
        self.fs.write_file(&config_path, &serde_json::to_string_pretty(&manifest)?)?;
        self.display.print_success(&format!("Updated version to {}", version));
        Ok(())
    }

    pub fn bump_version(&self, root: &Path, bump: VersionBump, dry_run: bool) -> Result<Version, ShaError> {
        let mut version = self.get_version(root)?;

        match bump {
            VersionBump::Major => {
                version.major += 1;
                version.minor = 0;
                version.patch = 0;
            }
            VersionBump::Minor => {
                version.minor += 1;
                version.patch = 0;
            }
            VersionBump::Patch => {
                version.patch += 1;
            }
            VersionBump::Auto => {
                let commits = self.get_auto_bump_commits()?;
                if commits.is_empty() {
                    self.display.print_warning("No new commits found since last tag. Version remains unchanged.");
                    return Ok(version);
                }
                let bump_type = determine_bump(&commits);
                match bump_type {
                    VersionBump::Major => {
                        version.major += 1;
                        version.minor = 0;
                        version.patch = 0;
                    }
                    VersionBump::Minor => {
                        version.minor += 1;
                        version.patch = 0;
                    }
                    _ => {
                        version.patch += 1;
                    }
                }
                self.display.print_info(&format!("Detected bump: {:?}", bump_type));
            }
        }

        self.set_version(root, &version, dry_run)?;
        Ok(version)
    }

    fn get_auto_bump_commits(&self) -> Result<Vec<GitCommit>, ShaError> {
        let last_tag = self.git.describe_tags()?.unwrap_or_default();
        let range = if last_tag.is_empty() {
            "HEAD".to_string()
        } else {
            format!("{}..HEAD", last_tag)
        };
        self.git.get_log(&range)
    }

    pub fn get_module_health(&self, root: &Path) -> Result<Vec<ModuleHealth>, ShaError> {
        let workspace = self.load_workspace(root)?;
        let mut health = Vec::new();

        for feature in &workspace.features {
            let kind = &feature.kind;
            let dir = root.join(kind.directory_name());

            if !self.fs.dir_exists(&dir) {
                continue;
            }

            let (status, details) = match kind {
                FeatureKind::MachineLearning => {
                    let hb = dir.join("heartbeat.json");
                    if self.fs.file_exists(&hb) {
                        let content = self.fs.read_file(&hb)?;
                        (HealthStatus::Active, content)
                    } else {
                        (HealthStatus::Idle, "No heartbeat.json found".to_string())
                    }
                }
                FeatureKind::WebFrontend | FeatureKind::WebBackend | FeatureKind::LandingPage => {
                    (HealthStatus::Running, "Use 'sha run' to check health endpoints".to_string())
                }
                FeatureKind::Research => {
                    let pdf = dir.join("main.pdf");
                    if self.fs.file_exists(&pdf) {
                        (HealthStatus::Complete, "Artifact main.pdf present".to_string())
                    } else {
                        (HealthStatus::Pending, "No PDF built yet".to_string())
                    }
                }
                FeatureKind::HardwareArduino | FeatureKind::HardwareMicroPython | FeatureKind::HardwareEmbeddedRust => {
                    (HealthStatus::Ready, "Toolchain configured".to_string())
                }
                _ => (HealthStatus::Unknown, "No pulse logic defined".to_string()),
            };

            health.push(ModuleHealth {
                name: feature.name.clone(),
                status,
                details,
            });
        }

        Ok(health)
    }

    pub fn env_get(&self, root: &Path, key: &str) -> Result<Option<String>, ShaError> {
        if !self.env.is_available() {
            return Err(ShaError::EnvChainNotInstalled);
        }
        let namespace = self.env_namespace(root)?;
        self.env.get_env(&namespace, key)
    }

    pub fn env_set(&self, root: &Path, key: &str, value: &str, dry_run: bool) -> Result<(), ShaError> {
        if dry_run {
            self.display.print_dry_run(&format!("Would set env var: {} = {}", key, value));
            return Ok(());
        }
        if !self.env.is_available() {
            return Err(ShaError::EnvChainNotInstalled);
        }
        let namespace = self.env_namespace(root)?;
        self.env.set_env(&namespace, key, value)
    }

    pub fn env_list(&self, root: &Path) -> Result<Vec<EnvVar>, ShaError> {
        if !self.env.is_available() {
            return Err(ShaError::EnvChainNotInstalled);
        }
        let namespace = self.env_namespace(root)?;
        self.env.list_envs(&namespace)
    }

    fn env_namespace(&self, root: &Path) -> Result<String, ShaError> {
        let workspace = self.load_workspace(root)?;
        Ok(format!("shastack-{}", workspace.name))
    }
}

fn determine_bump(commits: &[GitCommit]) -> VersionBump {
    for commit in commits {
        if commit.message.contains("BREAKING CHANGE:")
            || (commit.message.contains('!') && commit.message.contains(':'))
        {
            return VersionBump::Major;
        }
        if commit.message.starts_with("feat") {
            return VersionBump::Minor;
        }
    }
    VersionBump::Patch
}

// --- Template Constants ---

const ROOT_JUSTFILE: &str = r#"set shell := ["bash", "-uc"]

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

const ROOT_CI: &str = r#"name: Global CI Coordinator

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

const WEB_JUSTFILE: &str = r#"set shell := ["bash", "-uc"]

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

const WEB_CI: &str = r#"name: Web CI

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

const MOBILE_CI: &str = r#"name: Mobile CI

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

const RESEARCH_CI: &str = r#"name: Research CI

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

const HARDWARE_CI: &str = r#"name: Hardware CI

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

const LANDING_CI: &str = r#"name: Landing CI

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

const PYTHON_CI: &str = r#"name: Python CI

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

const DEFAULT_CI: &str = r#"name: Feature CI

on:
  push:
    paths:
      - 'FEATURE/**'
  pull_request:
    paths:
      - 'FEATURE/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: No module checks configured
        run: echo 'No CI template configured for FEATURE'
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::fs::RealFileSystem;
    use crate::adapters::git::RealGit;
    use crate::adapters::scaffold::RealScaffolder;
    use crate::adapters::env::RealEnv;
    use crate::adapters::display::RealDisplay;
    use tempfile::tempdir;

    fn setup() -> (WorkspaceUseCases<'static>, &'static RealFileSystem, &'static RealGit, &'static RealScaffolder, &'static RealEnv, &'static RealDisplay) {
        // This is a simplified test setup
        todo!("Implement test setup with real adapters")
    }

    #[test]
    fn test_feature_kind_from_name() {
        assert_eq!(FeatureKind::from_name("Web Frontend (Angular)"), FeatureKind::WebFrontend);
        assert_eq!(FeatureKind::from_name("ML (Python/Notebooks)"), FeatureKind::MachineLearning);
        assert_eq!(FeatureKind::from_name("CustomThing"), FeatureKind::Custom);
    }

    #[test]
    fn test_feature_kind_directory() {
        assert_eq!(FeatureKind::WebFrontend.directory_name(), "web");
        assert_eq!(FeatureKind::MobileApp.directory_name(), "mobile");
        assert_eq!(FeatureKind::Research.directory_name(), "research");
    }
}
