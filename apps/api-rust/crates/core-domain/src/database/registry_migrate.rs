use sea_orm::{ConnectionTrait, DbErr};

const REGISTRY_SCHEMA_SQL: &str = include_str!("registry_schema.sql");

/// Run migrations for the central registry database (orbis_registry.db).
/// This is separate from the per-year schema migrations.
pub async fn run_registry_migrations<C: ConnectionTrait>(db: &C) -> Result<(), DbErr> {
    for statement in REGISTRY_SCHEMA_SQL.split(';') {
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
