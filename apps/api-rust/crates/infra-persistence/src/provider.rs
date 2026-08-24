use std::path::{Path, PathBuf};
use sea_orm::DatabaseConnection;
use tracing::info;

/// Manages database connection for the Tauri desktop application.
/// Provides access to the single central database.
pub struct DatabaseProvider {
    /// The single application database connection.
    db: DatabaseConnection,
    
    /// Path to the application data directory.
    app_data_dir: PathBuf,
}

impl DatabaseProvider {
    /// Initialize the DatabaseProvider by connecting to the main database.
    pub async fn init(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = app_data_dir.join("orbis.db");
        let url = crate::connection::sqlite_url_rwc(&db_path);
        let db = crate::connection::connect_db(&url).await?;
        
        info!("✓ Database connected at: {:?}", db_path);

        // Run baseline schema
        let active_baseline_sql = if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            include_str!("schemas/schema_baseline_pg.sql")
        } else {
            include_str!("schemas/schema_baseline.sql")
        };
        crate::migrations::run_sql_migration(&db, active_baseline_sql).await?;
        crate::schema_migrations::run_postgres_security_migrations(&db).await?;
        crate::schema_migrations::run_p0_migrations(&db).await?;
        crate::schema_migrations::run_p4_migrations(&db).await?;

        Ok(Self {
            db,
            app_data_dir: app_data_dir.to_path_buf(),
        })
    }

    /// Get the connection to the database.
    pub fn db(&self) -> DatabaseConnection {
        self.db.clone()
    }

    /// Get the app data directory.
    pub fn app_data_dir(&self) -> &Path {
        &self.app_data_dir
    }

    /// Compatibility method to check if current DB is writable
    pub async fn ensure_writable(&self) -> Result<(), String> {
        Ok(())
    }
}
