use std::sync::Arc;
use std::path::Path;
use sea_orm::DatabaseConnection;

// Re-export provider from infra-persistence
pub use infra_persistence::provider::DatabaseProvider;
pub use shared_kernel::config::Config;

/// Shared application state passed to all Tauri commands.
#[derive(Clone)]
pub struct AppState {
    /// Database provider.
    pub provider: Arc<DatabaseProvider>,
    
    /// Global application configuration.
    pub config: Arc<Config>,
}

impl AppState {
    /// Initialize the AppState.
    pub async fn init(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = Arc::new(DatabaseProvider::init(app_data_dir).await?);
        let config = Arc::new(Config::load());
        
        Ok(Self { provider, config })
    }

    /// Accessor for the active year's database connection.
    pub async fn active_db(&self) -> DatabaseConnection {
        self.provider.active_db().await
    }
}
