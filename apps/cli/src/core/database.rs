use anyhow::{Context, Result, anyhow};
use orbis_app::services::SystemService;
use orbis_infra::migration::Migrator;
use orbis_infra::repositories::system_repo::SystemRepoImpl;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;
use std::time::Duration;

use crate::core::config::CliState;

#[derive(Debug, Default)]
pub struct SystemStatus {
    pub db_exists: bool,
    pub has_organization: bool,
    pub has_admin: bool,
    pub has_academic_years: bool,
    pub org_name: Option<String>,
}

impl SystemStatus {
    pub fn is_initialized(&self) -> bool {
        self.db_exists && self.has_organization && self.has_admin
    }

    pub fn is_setup_complete(&self) -> bool {
        self.is_initialized() && self.has_academic_years
    }
}

/// Connects with a bounded 2-second timeout so offline databases fail fast instead of hanging 30s.
pub async fn fast_connect(db_url: &str) -> Result<DatabaseConnection> {
    let mut opts = ConnectOptions::new(db_url);
    opts.connect_timeout(Duration::from_secs(2))
        .acquire_timeout(Duration::from_secs(2))
        .max_connections(2)
        .sqlx_logging(false);
    Database::connect(opts)
        .await
        .with_context(|| "Could not connect to database")
}

/// Inspects the configured database (from `CliState` or `ORBIS_SERVER_CONFIG_FILE`).
pub async fn get_full_status() -> SystemStatus {
    let db_url = match CliState::load() {
        Some(state) => state.host_database_url(),
        None => match std::env::var("DATABASE_URL") {
            Ok(u) if !u.trim().is_empty() => u,
            _ => match orbis_infra::config::resolve_database_url() {
                Ok(u) => u,
                Err(_) => return SystemStatus::default(),
            },
        },
    };

    let connect_url = if db_url.starts_with("sqlite:") {
        db_url.replace("mode=rwc", "mode=ro")
    } else {
        db_url
    };

    let Ok(db) = fast_connect(&connect_url).await else {
        return SystemStatus::default();
    };

    let system_repo = Arc::new(SystemRepoImpl { db });
    let system_service = SystemService::new(system_repo);

    match system_service.get_readiness().await {
        Ok(readiness) => SystemStatus {
            db_exists: true,
            has_organization: readiness.has_organization,
            has_admin: readiness.has_admin,
            has_academic_years: readiness.has_academic_calendar,
            org_name: readiness.org_name,
        },
        Err(_) => SystemStatus {
            db_exists: true,
            ..Default::default()
        },
    }
}

pub async fn get_system_status() -> (bool, bool) {
    let status = get_full_status().await;
    (status.is_initialized(), status.is_setup_complete())
}

pub async fn test_connection(db_url: &str) -> Result<()> {
    let db = Database::connect(db_url)
        .await
        .with_context(|| "Could not connect to database")?;
    db.execute(Statement::from_string(
        if db_url.starts_with("sqlite:") {
            DbBackend::Sqlite
        } else {
            DbBackend::Postgres
        },
        "SELECT 1",
    ))
    .await
    .context("Database ping query failed")?;
    Ok(())
}

pub async fn run_migrations(db_url: &str) -> Result<()> {
    let db = Database::connect(db_url)
        .await
        .with_context(|| "Failed to connect to database for migrations")?;
    Migrator::up(&db, None)
        .await
        .with_context(|| "Failed running schema migrations")?;
    Ok(())
}

pub async fn pending_migrations_count(db_url: &str) -> Result<usize> {
    let db = Database::connect(db_url).await?;
    let pending = Migrator::get_pending_migrations(&db).await?;
    Ok(pending.len())
}

pub async fn reset_database(db_url: &str) -> Result<()> {
    if db_url.starts_with("sqlite:") {
        let path_str = db_url
            .replace("sqlite://", "")
            .replace("?mode=rwc", "")
            .replace("?mode=ro", "");
        let path = std::path::Path::new(&path_str);
        if path.exists() {
            std::fs::remove_file(path)
                .with_context(|| format!("Failed to remove SQLite file {}", path.display()))?;
        }
        return Ok(());
    }

    let db = Database::connect(db_url)
        .await
        .map_err(|e| anyhow!("Failed to connect to PostgreSQL: {e}"))?;
    db.execute(Statement::from_string(
        DbBackend::Postgres,
        "DROP SCHEMA public CASCADE; CREATE SCHEMA public;",
    ))
    .await
    .map_err(|e| anyhow!("Failed to reset PostgreSQL schema: {e}"))?;
    Ok(())
}
