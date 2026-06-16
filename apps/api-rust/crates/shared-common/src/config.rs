use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub env: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub jwt_refresh_days: i64,
    pub solver_service_url: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        // Safe default fallbacks matching Go infrastructure settings
        let env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3002);
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://../../orbis.db?mode=rwc".to_string());
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "orbis_super_secure_secret_key_2026".to_string());
        let jwt_expiry_hours = env::var("JWT_EXPIRY_HOURS")
            .ok()
            .and_then(|h| h.parse().ok())
            .unwrap_or(24);
        let jwt_refresh_days = env::var("JWT_REFRESH_DAYS")
            .ok()
            .and_then(|d| d.parse().ok())
            .unwrap_or(7);
        let solver_service_url = env::var("SOLVER_SERVICE_URL").ok();

        Config {
            env,
            port,
            database_url,
            jwt_secret,
            jwt_expiry_hours,
            jwt_refresh_days,
            solver_service_url,
        }
    }
}
