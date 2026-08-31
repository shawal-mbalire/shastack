use crate::domain::errors::ShaError;
use crate::domain::ports::FileSystemPort;
use std::path::{Path, PathBuf};
use std::fs;

pub struct RealFileSystem;

impl FileSystemPort for RealFileSystem {
    fn read_file(&self, path: &Path) -> Result<String, ShaError> {
        fs::read_to_string(path).map_err(|e| ShaError::IoError(format!("Failed to read {}: {}", path.display(), e)))
    }

    fn write_file(&self, path: &Path, content: &str) -> Result<(), ShaError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content).map_err(|e| ShaError::IoError(format!("Failed to write {}: {}", path.display(), e)))
    }

    fn create_dir(&self, path: &Path) -> Result<(), ShaError> {
        fs::create_dir_all(path).map_err(|e| ShaError::IoError(format!("Failed to create dir {}: {}", path.display(), e)))
    }

    fn dir_exists(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn file_exists(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>, ShaError> {
        let entries: Vec<PathBuf> = fs::read_dir(path)
            .map_err(|e| ShaError::IoError(format!("Failed to read dir {}: {}", path.display(), e)))?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();
        Ok(entries)
    }

    fn rename(&self, from: &Path, to: &Path) -> Result<(), ShaError> {
        fs::rename(from, to).map_err(|e| ShaError::IoError(format!("Failed to rename {} to {}: {}", from.display(), to.display(), e)))
    }

    fn remove_file(&self, path: &Path) -> Result<(), ShaError> {
        fs::remove_file(path).map_err(|e| ShaError::IoError(format!("Failed to remove {}: {}", path.display(), e)))
    }

    fn current_dir(&self) -> Result<PathBuf, ShaError> {
        std::env::current_dir().map_err(|e| ShaError::IoError(e.to_string()))
    }
}
