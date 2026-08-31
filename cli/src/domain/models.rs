use semver::Version;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
    pub name: String,
    pub version: Version,
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feature {
    pub name: String,
    pub kind: FeatureKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureKind {
    WebFrontend,
    WebBackend,
    LandingPage,
    MobileApp,
    Research,
    MachineLearning,
    HardwareArduino,
    HardwareMicroPython,
    HardwareEmbeddedRust,
    Custom,
}

impl FeatureKind {
    pub fn from_name(name: &str) -> Self {
        match name {
            "Web Frontend (Angular)" => Self::WebFrontend,
            "Web Backend (Flask)" => Self::WebBackend,
            "Landing Page (Angular)" => Self::LandingPage,
            "Mobile App (Flutter)" => Self::MobileApp,
            "Research (LaTeX)" => Self::Research,
            "ML (Python/Notebooks)" => Self::MachineLearning,
            "Hardware (Arduino/C++)" => Self::HardwareArduino,
            "Hardware (MicroPython/uv)" => Self::HardwareMicroPython,
            "Hardware (Embedded Rust)" => Self::HardwareEmbeddedRust,
            _ => Self::Custom,
        }
    }

    pub fn directory_name(&self) -> &'static str {
        match self {
            Self::WebFrontend | Self::WebBackend => "web",
            Self::LandingPage => "landing",
            Self::MobileApp => "mobile",
            Self::Research => "research",
            Self::MachineLearning => "ml",
            Self::HardwareArduino | Self::HardwareMicroPython | Self::HardwareEmbeddedRust => "hardware",
            Self::Custom => "custom",
        }
    }

    pub fn all_names() -> Vec<&'static str> {
        vec![
            "Web Frontend (Angular)",
            "Web Backend (Flask)",
            "Landing Page (Angular)",
            "Mobile App (Flutter)",
            "Research (LaTeX)",
            "ML (Python/Notebooks)",
            "Hardware (Arduino/C++)",
            "Hardware (MicroPython/uv)",
            "Hardware (Embedded Rust)",
        ]
    }
}

#[derive(Debug, Clone)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionBump {
    Major,
    Minor,
    Patch,
    Auto,
}

#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ModuleHealth {
    pub name: String,
    pub status: HealthStatus,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Active,
    Idle,
    Running,
    Complete,
    Pending,
    Ready,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ModelEntry {
    pub model: String,
    pub weight_path: String,
    pub git_hash: String,
    pub timestamp: String,
}
