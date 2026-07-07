use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use sea_orm::{entity::prelude::*, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use tracing::info;
use serde::{Deserialize, Serialize};

/// Represents a single academic year with its associated database file.
/// Lives in `orbis_registry.db`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "academic_years")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub hijri_start_date: Option<String>,
    pub hijri_end_date: Option<String>,
    #[sea_orm(unique)]
    pub db_filename: String,
    pub status: String,
    pub is_current: i32,
    pub archived_at: Option<String>,
    pub archived_by: Option<String>,
    pub promotion_source_year_id: Option<String>,
    pub db_size_bytes: Option<i64>,
    pub student_count: Option<i32>,
    pub teacher_count: Option<i32>,
    pub class_count: Option<i32>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// Manages database connections for the Tauri desktop application.
/// Provides access to the central registry database and the active year database.
pub struct DatabaseProvider {
    /// Registry database connection (always open, pointing to orbis_registry.db).
    registry_db: DatabaseConnection,
    
    /// Current year's active database connection. Wrapped in RwLock for safe dynamic switching.
    active_db: RwLock<DatabaseConnection>,
    
    /// Currently active year ID from registry.
    active_year_id: RwLock<String>,
    
    /// Whether the active academic year is archived (read-only mode).
    is_archived: RwLock<bool>,
    
    /// Path to the application data directory.
    app_data_dir: PathBuf,
}

impl DatabaseProvider {
    /// Initialize the DatabaseProvider by connecting to the registry and resolving the active year.
    pub async fn init(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        // 1. Resolve registry DB path and connect
        let registry_db_path = app_data_dir.join("orbis_registry.db");
        let registry_url = crate::registry::sqlite_url_rwc(&registry_db_path);
        let registry_db = crate::connection::connect_registry_db(&registry_url).await?;
        info!("✅ Registry database connected at: {:?}", registry_db_path);

        // 2. Find current active year
        let years = Entity::find()
            .filter(Column::IsCurrent.eq(1))
            .all(&registry_db)
            .await?;

        let (active_db, year_id, is_archived) = if let Some(year) = years.first() {
            let years_dir = crate::registry::years_dir(app_data_dir);
            std::fs::create_dir_all(&years_dir)?;
            let db_path = crate::registry::year_db_path(app_data_dir, &year.db_filename);
            let url = if year.status == "archived" {
                crate::registry::sqlite_url_readonly(&db_path)
            } else {
                crate::registry::sqlite_url_rwc(&db_path)
            };
            
            let db = if year.status == "archived" {
                crate::connection::connect_archived(&url).await?
            } else {
                crate::connection::connect_year(&url).await?
            };
            
            info!("🗃️ Opened current year: {} ({})", year.name, year.db_filename);
            (db, year.id.clone(), year.status == "archived")
        } else {
            // Fallback for first launch or legacy
            let years_dir = crate::registry::years_dir(app_data_dir);
            std::fs::create_dir_all(&years_dir)?;
            
            let db_filename = crate::registry::year_db_filename("default");
            let db_path = years_dir.join(&db_filename);
            let url = crate::registry::sqlite_url_rwc(&db_path);
            let db = crate::connection::connect_year(&url).await?;
            
            info!("🆕 Created default year database at: {:?}", db_path);
            (db, String::new(), false)
        };

        Ok(Self {
            registry_db,
            active_db: RwLock::new(active_db),
            active_year_id: RwLock::new(year_id),
            is_archived: RwLock::new(is_archived),
            app_data_dir: app_data_dir.to_path_buf(),
        })
    }

    /// Get the connection to the registry database.
    pub fn registry(&self) -> &DatabaseConnection {
        &self.registry_db
    }

    /// Get the connection to the currently active year's database.
    pub async fn active_db(&self) -> DatabaseConnection {
        self.active_db.read().await.clone()
    }

    /// Get the current active academic year ID.
    pub async fn current_year_id(&self) -> String {
        self.active_year_id.read().await.clone()
    }

    /// Check if the current active year is archived.
    pub async fn is_current_archived(&self) -> bool {
        *self.is_archived.read().await
    }

    /// Create a fresh year database.
    pub async fn create_year_db(&self, db_filename: &str) -> Result<DatabaseConnection, String> {
        let db_path = crate::registry::year_db_path(&self.app_data_dir, db_filename);
        let url = crate::registry::sqlite_url_rwc(&db_path);
        let db = crate::connection::connect_year(&url)
            .await
            .map_err(|e| e.to_string())?;
        Ok(db)
    }

    /// Get the app data directory.
    pub fn app_data_dir(&self) -> &Path {
        &self.app_data_dir
    }

    /// Ensure that the active year database is writable.
    pub async fn ensure_writable(&self) -> Result<(), String> {
        if *self.is_archived.read().await {
            return Err("Cannot modify data in an archived year. Switch to an active year first.".to_string());
        }
        Ok(())
    }

    /// Switch the active database connection to another academic year.
    pub async fn switch_year(&self, year_id: &str) -> Result<(), String> {
        // Find the year details in the registry
        let year = Entity::find_by_id(year_id.to_string())
            .one(&self.registry_db)
            .await
            .map_err(|e| format!("Database query failed: {e}"))?
            .ok_or_else(|| format!("Academic year with ID '{year_id}' not found"))?;

        // Don't switch if already active
        let current_id = self.active_year_id.read().await.clone();
        if current_id == year_id {
            return Ok(());
        }

        let db_path = crate::registry::year_db_path(&self.app_data_dir, &year.db_filename);
        let is_archived = year.status == "archived";
        let url = if is_archived {
            crate::registry::sqlite_url_readonly(&db_path)
        } else {
            crate::registry::sqlite_url_rwc(&db_path)
        };

        let new_db = if is_archived {
            crate::connection::connect_archived(&url)
                .await
                .map_err(|e| format!("Failed to connect to archived DB: {e}"))?
        } else {
            crate::connection::connect_year(&url)
                .await
                .map_err(|e| format!("Failed to connect to active DB: {e}"))?
        };

        // Atomically update provider fields
        {
            let mut active_lock = self.active_db.write().await;
            *active_lock = new_db;
        }
        {
            let mut year_lock = self.active_year_id.write().await;
            *year_lock = year_id.to_string();
        }
        {
            let mut archived_lock = self.is_archived.write().await;
            *archived_lock = is_archived;
        }

        info!("🔀 Switched active year to: {} ({})", year.name, year.db_filename);
        Ok(())
    }
}
