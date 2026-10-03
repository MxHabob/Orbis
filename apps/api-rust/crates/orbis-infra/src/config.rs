//! Configuration and Environment Resolution for Orbis Infrastructure.
//!
//! Provides deterministic database path discovery shared across Desktop (Tauri),
//! HTTP Server (Axum), and CLI commands.

use std::path::PathBuf;

/// Resolve the canonical SQLite or PostgreSQL database connection URL.
///
/// Priority Order:
/// 1. `DATABASE_URL` environment variable (if explicitly defined)
/// 2. Active Desktop App SQLite database in user data dir (`%APPDATA%/com.mulverse.orbis.dev/orbis.db` or prod)
/// 3. Standard fallback `sqlite://orbis.db?mode=rwc`
pub fn resolve_database_url() -> String {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        let trimmed = url.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    // 1. Windows standard AppData Roaming paths
    if let Ok(appdata) = std::env::var("APPDATA") {
        let dev_db = PathBuf::from(&appdata)
            .join("com.mulverse.orbis.dev")
            .join("orbis.db");
        if is_non_empty_file(&dev_db) {
            return format!("sqlite://{}?mode=rwc", dev_db.display());
        }
        let prod_db = PathBuf::from(&appdata)
            .join("com.mulverse.orbis")
            .join("orbis.db");
        if is_non_empty_file(&prod_db) {
            return format!("sqlite://{}?mode=rwc", prod_db.display());
        }
    }

    // 2. macOS & Linux Application Support / Local Share paths
    if let Ok(home) = std::env::var("HOME") {
        let mac_db = PathBuf::from(&home)
            .join("Library/Application Support/com.mulverse.orbis.dev/orbis.db");
        if is_non_empty_file(&mac_db) {
            return format!("sqlite://{}?mode=rwc", mac_db.display());
        }
        let linux_db = PathBuf::from(&home).join(".local/share/com.mulverse.orbis.dev/orbis.db");
        if is_non_empty_file(&linux_db) {
            return format!("sqlite://{}?mode=rwc", linux_db.display());
        }
    }

    // 3. Current Working Directory fallback
    "sqlite://orbis.db?mode=rwc".to_string()
}

fn is_non_empty_file(path: &PathBuf) -> bool {
    path.is_file()
        && std::fs::metadata(path)
            .map(|metadata| metadata.len() > 0)
            .unwrap_or(false)
}
