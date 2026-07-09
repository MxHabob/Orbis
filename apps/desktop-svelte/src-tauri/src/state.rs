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

    /// Verify RBAC permissions for the current active user session in Tauri.
    pub async fn check_tauri_permission(
        &self,
        db: &DatabaseConnection,
        permission: domain_iam::services::authz::Permission,
        context: Option<&domain_iam::services::authz::AuthzContext>,
    ) -> Result<(), shared_kernel::errors::AppError> {
        use sea_orm::{EntityTrait, QueryOrder, QueryFilter, ColumnTrait};
        
        let latest_session = domain_iam::entities::user_session::Entity::find()
            .order_by_desc(domain_iam::entities::user_session::Column::CreatedAt)
            .one(db)
            .await?
            .ok_or_else(|| shared_kernel::errors::AppError::unauthorized("No active session found"))?;

        let user = domain_iam::entities::user::Entity::find_by_id(latest_session.user_id)
            .one(db)
            .await?
            .ok_or_else(|| shared_kernel::errors::AppError::unauthorized("Active session user not found"))?;

        let authz = domain_iam::services::authz::AuthzService::new();
        authz.check_permission(db, &user, permission, context).await?;

        Ok(())
    }
}
