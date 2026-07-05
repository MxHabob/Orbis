//! DatabaseProvider — transparent multi-year database abstraction.
//!
//! This module provides the `DatabaseProvider` which manages the current year's
//! database connection behind a `RwLock`. All existing Tauri commands access
//! the current year's `Service` through `provider.current().await` without
//! any knowledge of year switching mechanics.
//!
//! # Design Rationale
//!
//! Instead of modifying every Tauri command to handle year switching, we introduce
//! a single abstraction layer. Commands call `state.service().await` which returns
//! the current year's Service. Switching years swaps the inner Service atomically,
//! and subsequent reads see the new year's data.
//!
//! Only one year database is open at a time (the active year). Archived years
//! are opened on-demand for read-only access when the user switches to view them.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::config::Config;
use crate::database::registry;
use crate::repository::Repository;
use crate::service::Service;

/// Provides transparent access to the current year's Service.
///
/// All Tauri commands use `provider.current()` to get the active Service.
/// Year switching is handled internally and is invisible to callers.
pub struct DatabaseProvider {
    /// The current year's fully-initialized Service (behind RwLock for switching).
    current_service: RwLock<Arc<Service>>,

    /// The registry database connection (always open, never switched).
    registry_db: DatabaseConnection,

    /// Currently active year ID.
    current_year_id: RwLock<String>,

    /// Whether the current year is archived (read-only mode).
    is_archived: RwLock<bool>,

    /// Path to the app data directory (for resolving year DB paths).
    app_data_dir: PathBuf,
}

impl DatabaseProvider {
    /// Initialize the provider by connecting to the registry and opening
    /// the current active year's database.
    ///
    /// If no academic years exist yet (first launch), creates a default year.
    pub async fn init(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        // 1. Connect to the registry database
        let registry_db = registry::connect_registry(app_data_dir).await?;
        info!("✅ Registry database connected");

        // 2. Find the current active year
        use crate::repository::academic_year::AcademicYearRepo;
        // We need at least one org — get the first one if it exists
        // For first launch, we'll handle the case of no years in the caller
        let years = crate::entities::academic_year::Entity::find()
            .filter(crate::entities::academic_year::Column::IsCurrent.eq(1))
            .all(&registry_db)
            .await?;

        let (year_service, year_id, is_archived) = if let Some(year) = years.first() {
            // Open the current year's database
            let db_path = registry::year_db_path(app_data_dir, &year.db_filename);
            let service = Self::create_service_for_path(&db_path).await?;
            info!("🗃️ Opened current year: {} ({})", year.name, year.db_filename);
            (service, year.id.clone(), year.status == "archived")
        } else {
            // No years exist yet — open legacy orbis.db if it exists, else create fresh
            let legacy_path = app_data_dir.join("orbis.db");
            if legacy_path.exists() {
                let service = Self::create_service_for_path(&legacy_path).await?;
                info!("📦 Opened legacy database (orbis.db) — no academic years registered");
                (service, String::new(), false)
            } else {
                // First launch — create the years directory
                let years_dir = registry::years_dir(app_data_dir);
                std::fs::create_dir_all(&years_dir)?;

                // Create a default year database
                let db_filename = registry::year_db_filename("default");
                let db_path = years_dir.join(&db_filename);
                let service = Self::create_service_for_path(&db_path).await?;
                info!("🆕 Created default year database");
                (service, String::new(), false)
            }
        };

        Ok(Self {
            current_service: RwLock::new(Arc::new(year_service)),
            registry_db,
            current_year_id: RwLock::new(year_id),
            is_archived: RwLock::new(is_archived),
            app_data_dir: app_data_dir.to_path_buf(),
        })
    }

    /// Get the current year's Service. This is the primary method used by all commands.
    ///
    /// Returns an `Arc<Service>` that can be used for the duration of a single command.
    /// The Arc ensures the Service remains valid even if a year switch happens concurrently.
    pub async fn current(&self) -> Arc<Service> {
        self.current_service.read().await.clone()
    }

    /// Get the registry database connection (for year management operations).
    pub fn registry(&self) -> &DatabaseConnection {
        &self.registry_db
    }

    /// Get the currently active year ID.
    pub async fn current_year_id(&self) -> String {
        self.current_year_id.read().await.clone()
    }

    /// Check if the current year is archived (read-only).
    pub async fn is_current_archived(&self) -> bool {
        *self.is_archived.read().await
    }

    /// Get the app data directory path.
    pub fn app_data_dir(&self) -> &Path {
        &self.app_data_dir
    }

    /// Switch to a different academic year.
    ///
    /// This atomically swaps the current Service to point at the new year's database.
    /// Active year databases are opened read-write; archived years are read-only.
    pub async fn switch_year(&self, year_id: &str) -> Result<(), String> {
        use crate::repository::academic_year::AcademicYearRepo;

        // Look up the year in the registry
        let year = AcademicYearRepo::get_by_id(&self.registry_db, year_id)
            .await
            .map_err(|e| format!("Failed to look up year: {}", e))?
            .ok_or_else(|| "Academic year not found".to_string())?;

        // Don't switch if we're already on this year
        let current_id = self.current_year_id.read().await.clone();
        if current_id == year_id {
            return Ok(());
        }

        // Open the year's database
        let db_path = registry::year_db_path(&self.app_data_dir, &year.db_filename);
        let is_archived = year.status == "archived";

        let new_service = if is_archived {
            Self::create_service_for_archived_path(&db_path).await?
        } else {
            Self::create_service_for_path(&db_path)
                .await
                .map_err(|e| e.to_string())?
        };

        // Atomically swap the service
        {
            let mut service = self.current_service.write().await;
            *service = Arc::new(new_service);
        }
        {
            let mut id = self.current_year_id.write().await;
            *id = year_id.to_string();
        }
        {
            let mut archived = self.is_archived.write().await;
            *archived = is_archived;
        }

        info!(
            "🔀 Switched to year: {} ({}) [{}]",
            year.name,
            year.db_filename,
            if is_archived { "archived/read-only" } else { "active" }
        );

        Ok(())
    }

    /// Create a fresh year database and return a Service pointing to it.
    /// Used during promotion to create the target year's database.
    pub async fn create_year_db(&self, db_filename: &str) -> Result<Service, String> {
        let db_path = registry::year_db_path(&self.app_data_dir, db_filename);
        Self::create_service_for_path(&db_path)
            .await
            .map_err(|e| e.to_string())
    }

    // ── Internal helpers ────────────────────────────────────────────

    async fn create_service_for_path(
        db_path: &Path,
    ) -> Result<Service, Box<dyn std::error::Error>> {
        let url = crate::database::registry::sqlite_url_for_service(db_path);
        let db = crate::database::connect(&url).await?;
        let repo = Repository::new(db);
        let cfg = Config::load();
        Ok(Service::new(repo, cfg))
    }

    async fn create_service_for_archived_path(
        db_path: &Path,
    ) -> Result<Service, String> {
        let db = registry::open_archived_database(db_path)
            .await
            .map_err(|e| format!("Failed to open archived database: {}", e))?;
        let repo = Repository::new(db);
        let cfg = Config::load();
        Ok(Service::new(repo, cfg))
    }
}
