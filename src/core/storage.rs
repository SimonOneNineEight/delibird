use std::{fs, path::PathBuf};

use super::{
    error::{AppError, AppResult},
    task::task::Task,
};

#[derive(Debug)]
pub struct Storage {
    pub file_path: PathBuf,
}

impl Storage {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn save(&self, tasks: &[Task]) -> AppResult<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).map_err(|err| AppError::SaveFailed {
                path: self.file_path.display().to_string(),
                reason: format!("Cannot crete directory: {}", err),
            })?;
        }

        let json = serde_json::to_string_pretty(tasks).map_err(|err| AppError::SaveFailed {
            path: self.file_path.display().to_string(),
            reason: format!("Failed to serialize tasks: {}", err),
        })?;

        fs::write(&self.file_path, json).map_err(|err| AppError::SaveFailed {
            path: self.file_path.display().to_string(),
            reason: err.to_string(),
        })
    }

    pub fn load(&self) -> AppResult<Vec<Task>> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let contents = fs::read_to_string(&self.file_path).map_err(|err| AppError::LoadFailed {
            path: self.file_path.display().to_string(),
            reason: format!("Failed to read task file: {}", err),
        })?;

        serde_json::from_str(&contents).map_err(|err| AppError::LoadFailed {
            path: self.file_path.display().to_string(),
            reason: format!("JSON parsing failed: {}", err),
        })
    }
}
