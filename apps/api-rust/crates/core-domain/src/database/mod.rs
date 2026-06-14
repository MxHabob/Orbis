use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, ConnectionTrait};
use tracing::info;

pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    
    // Configure pool settings
    opt.max_connections(10)
       .min_connections(2)
       .connect_timeout(Duration::from_secs(5))
       .acquire_timeout(Duration::from_secs(5))
       .idle_timeout(Duration::from_secs(30))
       .max_lifetime(Duration::from_secs(1800))
       .sqlx_logging(true)
       .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;
    info!("✅ Connected to SQLite database with SeaORM");

    // Enforce WAL (Write-Ahead Logging) and performance pragmas
    db.execute_unprepared(
        "PRAGMA journal_mode=WAL; \
         PRAGMA synchronous=NORMAL; \
         PRAGMA foreign_keys=ON; \
         PRAGMA cache_size=-64000; \
         PRAGMA busy_timeout=5000;"
    ).await?;
    
    info!("🚀 SQLite WAL mode & performance pragmas successfully configured");

    Ok(db)
}
