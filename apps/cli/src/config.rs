use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub db_url: String,
    pub install_path: String,
}

impl CliConfig {
    pub fn default() -> Self {
        Self {
            db_url: "sqlite://orbis.db?mode=rwc".to_string(),
            install_path: "./".to_string(),
        }
    }
}
