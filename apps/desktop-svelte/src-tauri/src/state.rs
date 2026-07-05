use std::sync::Arc;
use std::path::Path;
use core_domain::{
    database::provider::DatabaseProvider,
    service::Service,
};

/// Build a SeaORM/sqlx-compatible SQLite URL from an absolute filesystem path.
/// Windows paths must use forward slashes and the `sqlite:///C:/...` form.
pub fn sqlite_database_url(db_path: &std::path::Path) -> String {
    let path = db_path.to_string_lossy().replace('\\', "/");
    if path.starts_with('/') {
        format!("sqlite://{path}?mode=rwc")
    } else {
        format!("sqlite:///{path}?mode=rwc")
    }
}

/// Shared application state passed to all Tauri commands.
///
/// Uses `DatabaseProvider` for transparent multi-year database access.
/// All commands call `state.service().await` to get the current year's Service,
/// without any knowledge of year switching mechanics.
#[derive(Clone)]
pub struct AppState {
    /// The database provider — manages current year's Service behind a RwLock.
    pub provider: Arc<DatabaseProvider>,
}

impl AppState {
    /// Initialize AppState by setting up the DatabaseProvider.
    ///
    /// The provider connects to the registry database, finds the current active
    /// academic year, and opens its database. If no years exist, it handles
    /// first-launch scenarios (legacy orbis.db or fresh install).
    pub async fn init(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = DatabaseProvider::init(app_data_dir).await?;

        Ok(Self {
            provider: Arc::new(provider),
        })
    }

    /// Get the current year's Service. Used by all Tauri commands.
    ///
    /// This is the primary access point. Returns an `Arc<Service>` that remains
    /// valid for the duration of a single command, even if a year switch occurs
    /// concurrently.
    pub async fn service(&self) -> Arc<Service> {
        self.provider.current().await
    }
}
