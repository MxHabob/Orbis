//! Schema migration runner.
//!
//! Runs SQL migration files statement-by-statement.
//! Uses `include_str!` to embed SQL at compile time — no runtime file I/O.

use sea_orm::{ConnectionTrait, DbErr};

/// Run a SQL migration string by splitting on `;` and executing each statement.
///
/// - Strips comments (lines starting with `--`)
/// - Skips empty statements
/// - Executes each statement individually
pub async fn run_sql_migration<C: ConnectionTrait>(db: &C, sql: &str) -> Result<(), DbErr> {
    for statement in sql.split(';') {
        let cleaned: String = statement
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with("--"))
            .collect::<Vec<_>>()
            .join("\n");
        let stmt = cleaned.trim();
        if stmt.is_empty() {
            continue;
        }
        db.execute_unprepared(stmt).await?;
    }
    Ok(())
}

/// Run a SQL cleanup migration — ignores errors (for DROP/ALTER statements
/// that may fail if the column/table doesn't exist).
pub async fn run_sql_cleanup<C: ConnectionTrait>(db: &C, sql: &str) {
    for statement in sql.split(';') {
        let cleaned: String = statement
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with("--"))
            .collect::<Vec<_>>()
            .join("\n");
        let stmt = cleaned.trim();
        if stmt.is_empty() {
            continue;
        }
        let _ = db.execute_unprepared(stmt).await;
    }
}
