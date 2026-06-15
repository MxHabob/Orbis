use sea_orm::{ConnectionTrait, DbErr};

const CORE_SCHEMA_SQL: &str = include_str!("core_schema.sql");
const SCHEMA_SQL: &str = include_str!("schema.sql");

pub async fn run_migrations<C: ConnectionTrait>(db: &C) -> Result<(), DbErr> {
    for sql in [CORE_SCHEMA_SQL, SCHEMA_SQL] {
        for statement in sql.split(';') {
            let stmt = statement.trim();
            if stmt.is_empty() || stmt.starts_with("--") {
                continue;
            }
            db.execute_unprepared(stmt).await?;
        }
    }
    Ok(())
}
