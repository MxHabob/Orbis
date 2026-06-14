// ──────────────────────────────────────────────────
// Error helper: converts any Result<T, String> into
// a Tauri-serializable error so the frontend receives
// a typed error object instead of a raw string.
// ──────────────────────────────────────────────────
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandError {
    pub message: String,
    pub code: Option<String>,
}

impl CommandError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
        }
    }

    pub fn with_code(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: Some(code.into()),
        }
    }
}

impl From<String> for CommandError {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for CommandError {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Convenience type alias used by every Tauri command.
pub type CmdResult<T> = Result<T, CommandError>;
