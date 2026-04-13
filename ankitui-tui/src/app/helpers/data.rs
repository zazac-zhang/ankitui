//! Data management helpers
//!
//! Utility functions for data import, export, backup, and restore operations.

use crate::utils::error::{TuiError, TuiResult};
use chrono::Utc;
use std::path::Path;

/// Validate that a data directory exists and is accessible
pub fn validate_data_dir(data_dir: &Path) -> TuiResult<()> {
    if !data_dir.exists() {
        return Err(TuiError::State {
            message: format!("Data directory does not exist: {:?}", data_dir),
        });
    }

    if !data_dir.is_dir() {
        return Err(TuiError::State {
            message: format!("Data path is not a directory: {:?}", data_dir),
        });
    }

    Ok(())
}

/// Get the default data directory for AnkiTUI
pub fn get_default_data_dir() -> std::path::PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("ankitui")
}

/// Create a timestamped backup filename
pub fn create_backup_filename() -> String {
    format!("backup_{}.toml", Utc::now().format("%Y%m%d_%H%M%S"))
}

/// Validate that an import file exists and is readable
pub fn validate_import_file(path: &Path) -> TuiResult<()> {
    if !path.exists() {
        return Err(TuiError::State {
            message: format!("Import file does not exist: {:?}", path),
        });
    }

    if !path.is_file() {
        return Err(TuiError::State {
            message: format!("Import path is not a file: {:?}", path),
        });
    }

    Ok(())
}

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir_exists(path: &Path) -> TuiResult<()> {
    std::fs::create_dir_all(path).map_err(|e| TuiError::State {
        message: format!("Failed to create directory {:?}: {}", path, e),
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_backup_filename() {
        let filename = create_backup_filename();
        assert!(filename.starts_with("backup_"));
        assert!(filename.ends_with(".toml"));
    }

    #[test]
    fn test_get_default_data_dir() {
        let dir = get_default_data_dir();
        assert!(dir.ends_with("ankitui"));
    }
}
