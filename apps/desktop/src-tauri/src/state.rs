use std::sync::Arc;
use core_domain::{
    config::Config,
    database,
    repository::Repository,
    service::Service,
};

/// Shared application state passed to all Tauri commands.
/// Wraps the full Service layer for direct, zero-network access.
#[derive(Clone)]
pub struct AppState {
    pub service: Arc<Service>,
}

impl AppState {
    /// Initialize AppState by connecting to the SQLite database and building
    /// the full service layer. Must be called inside an async context.
    pub async fn init(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cfg = Config::load();
        cfg.database_url = database_url.to_string();
        // Local embedded server: use a fixed port for potential dev-mode HTTP fallback
        cfg.port = 3002;

        let db = database::connect(database_url).await?;
        let repo = Repository::new(db);
        let service = Service::new(repo, cfg);

        Ok(Self {
            service: Arc::new(service),
        })
    }
}
