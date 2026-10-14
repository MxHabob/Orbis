//! Configuration and Environment Resolution for Orbis Infrastructure.
//!
//! Provides explicit server-side database configuration.

/// Resolve the database URL configured for the API server.
///
/// Server deployments must configure their own database and must never fall back
/// to a per-user Desktop SQLite file.
pub fn resolve_database_url() -> Result<String, String> {
    let url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be configured for the Orbis API server".to_string())?;
    let url = url.trim();
    if url.is_empty() {
        return Err("DATABASE_URL cannot be empty for the Orbis API server".to_string());
    }
    Ok(url.to_string())
}
