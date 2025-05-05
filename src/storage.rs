use std::{fs, path::PathBuf};

use crate::task::Task;

#[derive(Debug)]
pub struct Storage {
    pub file_path: PathBuf,
}

impl Storage {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn save(&self, tasks: &[Task]) -> Result<(), String> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("Failed to serialize on save: {}", err))?;
        }

        let json = serde_json::to_string_pretty(tasks)
            .map_err(|err| format!("Failed to serialize tasks: {}", err))?;

        fs::write(&self.file_path, json)
            .map_err(|err| format!("Failed to write task to file: {}", err))
    }

    pub fn load(&self) -> Result<Vec<Task>, String> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let contents = fs::read_to_string(&self.file_path)
            .map_err(|err| format!("Failed to read task file: {}", err))?;

        serde_json::from_str(&contents)
            .map_err(|err| format!("Failed to convert to json when load task: {}", err))
    }
}
