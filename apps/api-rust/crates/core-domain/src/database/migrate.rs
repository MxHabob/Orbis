use sea_orm::{ConnectionTrait, DbErr};

const CORE_SCHEMA_SQL: &str = include_str!("core_schema.sql");
const SCHEMA_SQL: &str = include_str!("schema.sql");
const SCHEMA_CLEANUP_SQL: &str = include_str!("schema_cleanup.sql");

pub async fn run_migrations<C: ConnectionTrait>(db: &C) -> Result<(), DbErr> {
    for sql in [CORE_SCHEMA_SQL, SCHEMA_SQL] {
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
    }

    // Drop legacy scheduling columns superseded by Timefold + rooms table
    for statement in SCHEMA_CLEANUP_SQL.split(';') {
        let stmt = statement.trim();
        if stmt.is_empty() || stmt.starts_with("--") {
            continue;
        }
        let _ = db.execute_unprepared(stmt).await;
    }

    Ok(())
}
