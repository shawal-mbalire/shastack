use std::fmt;

#[derive(Debug)]
pub enum ShaError {
    NotAWorkspace,
    WorkspaceAlreadyExists(String),
    FeatureAlreadyExists(String),
    FeatureNotFound(String),
    InvalidConfig(String),
    VersionParseError(String),
    GitError(String),
    IoError(String),
    ScaffoldError(String),
    MissingTool(String),
    EnvChainNotInstalled,
    EnvChainError(String),
    NetworkError(String),
    DryRun,
}

impl fmt::Display for ShaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAWorkspace => write!(f, "Not a shastack workspace (no .sha/config.json found)"),
            Self::WorkspaceAlreadyExists(name) => write!(f, "Directory {} already exists and is not empty", name),
            Self::FeatureAlreadyExists(name) => write!(f, "Feature {} already exists", name),
            Self::FeatureNotFound(name) => write!(f, "Feature {} not found", name),
            Self::InvalidConfig(msg) => write!(f, "Invalid config: {}", msg),
            Self::VersionParseError(msg) => write!(f, "Version parse error: {}", msg),
            Self::GitError(msg) => write!(f, "Git error: {}", msg),
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::ScaffoldError(msg) => write!(f, "Scaffold error: {}", msg),
            Self::MissingTool(tool) => write!(f, "Missing required tool: {}", tool),
            Self::EnvChainNotInstalled => write!(f, "envchain is not installed. Install it with: brew install envchain"),
            Self::EnvChainError(msg) => write!(f, "envchain error: {}", msg),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::DryRun => write!(f, "Dry run mode - no changes made"),
        }
    }
}

impl std::error::Error for ShaError {}

impl From<std::io::Error> for ShaError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e.to_string())
    }
}

impl From<serde_json::Error> for ShaError {
    fn from(e: serde_json::Error) -> Self {
        Self::InvalidConfig(e.to_string())
    }
}

impl From<anyhow::Error> for ShaError {
    fn from(e: anyhow::Error) -> Self {
        Self::IoError(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for ShaError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::IoError(e.to_string())
    }
}
