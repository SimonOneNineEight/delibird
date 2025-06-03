use std::time::{Duration, Instant};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum AppError {
    // Storage Errors
    SaveFailed {
        path: String,
        reason: String,
    },
    LoadFailed {
        path: String,
        reason: String,
    },
    ConfigDirCreation(String),
    FilePermissions {
        path: String,
    },

    // Input Validation Errors
    EmptyTaskTitle,
    InvalidTitle {
        reason: String,
    },
    InvalidDate {
        input: String,
        expected_format: String,
    },
    InvalidDescription {
        reason: String,
    },
    TaskNotFound {
        id: Uuid,
    },

    // System Errors
    TerminalUnavailable,
    EventHandlingFailed(String),
    RenderFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Fatal,
}

impl AppError {
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppError::EmptyTaskTitle
            | AppError::InvalidTitle { .. }
            | AppError::InvalidDate { .. }
            | AppError::InvalidDescription { .. } => ErrorSeverity::Warning,
            AppError::SaveFailed { .. }
            | AppError::LoadFailed { .. }
            | AppError::FilePermissions { .. } => ErrorSeverity::Error,
            AppError::TerminalUnavailable => ErrorSeverity::Fatal,
            AppError::ConfigDirCreation(_) => ErrorSeverity::Warning,
            AppError::TaskNotFound { .. } => ErrorSeverity::Info,
            AppError::EventHandlingFailed(_) | AppError::RenderFailed(_) => ErrorSeverity::Warning,
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            AppError::SaveFailed { reason, .. } => {
                format!("Failed to save tasks: {}", reason)
            }
            AppError::LoadFailed { reason, .. } => {
                format!("Failed to load tasks: {}", reason)
            }
            AppError::ConfigDirCreation(reason) => {
                format!("Cannot create config directory: {}", reason)
            }
            AppError::FilePermissions { path } => {
                format!("No permission to write to: {}", path)
            }
            AppError::EmptyTaskTitle => "Task title cannot be empty".to_string(),
            AppError::InvalidDate {
                input,
                expected_format,
            } => {
                format!(
                    "Invalid date '{}'. Expected format: {}",
                    input, expected_format
                )
            }
            AppError::InvalidTitle { reason } => {
                format!("Invalid Title: {}", reason)
            }
            AppError::InvalidDescription { reason } => {
                format!("Invalid description: {}", reason)
            }
            AppError::TaskNotFound { id } => {
                format!("Task not found: {}", id)
            }
            AppError::TerminalUnavailable => {
                "Terminal is not available - cannot continue".to_string()
            }
            AppError::EventHandlingFailed(reason) => {
                format!("Input handling error: {}", reason)
            }
            AppError::RenderFailed(reason) => {
                format!("Display error: {}", reason)
            }
        }
    }

    pub fn debug_info(&self) -> String {
        format!("{:?}", self)
    }

    pub fn is_storage_error(&self) -> bool {
        matches!(
            self,
            AppError::SaveFailed { .. }
                | AppError::LoadFailed { .. }
                | AppError::ConfigDirCreation(_)
                | AppError::FilePermissions { .. }
        )
    }

    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            AppError::EmptyTaskTitle
                | AppError::InvalidDate { .. }
                | AppError::InvalidDescription { .. }
        )
    }

    pub fn is_fatal(&self) -> bool {
        self.severity() == ErrorSeverity::Fatal
    }
}

#[derive(Debug, Default)]
pub struct ErrorState {
    pub current_error: Option<AppError>,
    pub show_error_popup: bool,
    pub error_timestamp: Option<Instant>,
    pub error_history: Vec<(AppError, Instant)>,
}

impl ErrorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_error(&mut self, error: AppError) {
        let now = Instant::now();

        self.error_history.push((error.clone(), now));

        if self.error_history.len() > 10 {
            self.error_history.remove(0);
        }

        match error.severity() {
            ErrorSeverity::Info => {
                self.current_error = None;
                self.show_error_popup = false;
            }
            ErrorSeverity::Warning => {
                self.current_error = Some(error);
                self.show_error_popup = false;
                self.error_timestamp = Some(now);
            }
            ErrorSeverity::Error => {
                self.current_error = Some(error);
                self.show_error_popup = true;
                self.error_timestamp = Some(now);
            }
            ErrorSeverity::Fatal => {
                self.current_error = Some(error);
                self.show_error_popup = true;
                self.error_timestamp = Some(now);
            }
        }
    }

    pub fn clear_error(&mut self) {
        self.current_error = None;
        self.show_error_popup = false;
        self.error_timestamp = None;
    }

    pub fn has_error(&self) -> bool {
        self.current_error.is_some()
    }

    pub fn should_show_popup(&self) -> bool {
        self.show_error_popup && self.current_error.is_some()
    }

    pub fn should_auto_dismiss(&self) -> bool {
        if let (Some(error), Some(timestamp)) = (&self.current_error, self.error_timestamp) {
            if error.severity() == ErrorSeverity::Warning {
                return timestamp.elapsed() > Duration::from_secs(5);
            }
        }
        false
    }

    pub fn update(&mut self) {
        if self.should_auto_dismiss() {
            self.clear_error();
        }
    }

    pub fn current_message(&self) -> Option<String> {
        self.current_error
            .as_ref()
            .map(|error| error.user_message())
    }

    pub fn current_severity(&self) -> Option<ErrorSeverity> {
        self.current_error.as_ref().map(|error| error.severity())
    }

    pub fn dismiss(&mut self) {
        if let Some(error) = &self.current_error {
            if !error.is_fatal() {
                self.clear_error();
            }
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::SaveFailed {
            path: "unknown".to_string(),
            reason: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SaveFailed {
            path: "unknown".to_string(),
            reason: format!("JSON error: {}", err),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
