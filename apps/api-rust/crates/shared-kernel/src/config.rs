//! Application configuration.
//!
//! Migrated from `shared-common` with additions for Autopilot and AI.
//! All values have safe defaults for development. Production values
//! are set via environment variables.

use std::env;

/// Central application configuration.
#[derive(Clone, Debug)]
pub struct Config {
    // ── Core ────────────────────────────────────────────────────
    pub env: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub jwt_refresh_days: i64,
    pub public_registration_enabled: bool,

    // ── Scheduling ──────────────────────────────────────────────
    pub solver_service_url: Option<String>,

    // ── AI / LLM ────────────────────────────────────────────────
    pub llm_provider: String,
    pub ollama_url: String,
    pub ollama_model: String,
    pub gemini_api_key: String,
}

impl Config {
    pub fn load() -> Self {
        Config {
            // Core
            env: env::var("APP_ENV")
                .or_else(|_| env::var("ORBIS_ENV"))
                .unwrap_or_else(|_| "development".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3002),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://../../orbis.db?mode=rwc".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_default(),
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .ok()
                .and_then(|h| h.parse().ok())
                .unwrap_or(24),
            jwt_refresh_days: env::var("JWT_REFRESH_DAYS")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(7),
            public_registration_enabled: env::var("PUBLIC_REGISTRATION_ENABLED")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or_else(|| !env::var("APP_ENV")
                    .or_else(|_| env::var("ORBIS_ENV"))
                    .map(|value| value.eq_ignore_ascii_case("production"))
                    .unwrap_or(false)),

            // Scheduling
            solver_service_url: env::var("SOLVER_SERVICE_URL")
                .ok()
                .filter(|s| !s.is_empty())
                .or_else(|| Some("http://localhost:8081".to_string())),

            // AI / LLM — local-first (ADQ-3)
            llm_provider: env::var("LLM_PROVIDER")
                .unwrap_or_else(|_| "ollama".to_string()),
            ollama_url: env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:11434".to_string()),
            ollama_model: env::var("OLLAMA_MODEL")
                .unwrap_or_else(|_| "llama3.1:8b".to_string()),
            gemini_api_key: env::var("GEMINI_API_KEY")
                .unwrap_or_else(|_| "".to_string()),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.env.eq_ignore_ascii_case("production") {
            if self.jwt_secret.len() < 32 {
                return Err("JWT_SECRET must be set to at least 32 characters in production".to_string());
            }
            if self.database_url.trim().is_empty() {
                return Err("DATABASE_URL must be set in production".to_string());
            }
        }

        Ok(())
    }

    pub fn is_development(&self) -> bool {
        self.env == "development"
    }
}
