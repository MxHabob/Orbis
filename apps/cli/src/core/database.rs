use sea_orm::{Database, Statement, DatabaseBackend, ConnectionTrait};

/// Returns (db_exists, setup_completed)
pub async fn get_system_status() -> (bool, bool) {
    let config = crate::core::config::CliConfig::default();
    let db_url = config.db_url.replace("mode=rwc", "mode=ro");

    let Ok(db) = Database::connect(&db_url).await else {
        return (false, false);
    };
    
    // Check if organizations exist. If not, the CLI init hasn't been done.
    let org_res = db.query_one(Statement::from_string(
        DatabaseBackend::Sqlite,
        "SELECT COUNT(*) as cnt FROM organizations;".to_owned()
    )).await;
    
    let is_initialized = if let Ok(Some(row)) = org_res {
        let count: i64 = row.try_get("", "cnt").unwrap_or(0);
        count > 0
    } else {
        false
    };

    if !is_initialized {
        return (false, false); // Treat as not initialized
    }

    // Check if there are any academic_years, meaning Svelte onboarding has been done
    let res = db.query_one(Statement::from_string(
        DatabaseBackend::Sqlite,
        "SELECT COUNT(*) as cnt FROM academic_years;".to_owned()
    )).await;
    
    if let Ok(Some(row)) = res {
        let count: i64 = row.try_get("", "cnt").unwrap_or(0);
        return (true, count > 0);
    }
    
    (true, false)
}
