use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager.has_column("grades", "is_absent").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Grades::Table)
                        .add_column(
                            ColumnDef::new(Grades::IsAbsent)
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_column("grades", "is_absent").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Grades::Table)
                        .drop_column(Grades::IsAbsent)
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}

#[derive(Iden)]
enum Grades {
    Table,
    IsAbsent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ConnectionTrait, Database, DatabaseBackend, Statement};

    #[tokio::test]
    async fn migration_adds_absence_flag_with_false_default_to_existing_grades() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared(
            "CREATE TABLE grades (id INTEGER PRIMARY KEY, is_excused BOOLEAN NOT NULL DEFAULT FALSE);
             INSERT INTO grades (id, is_excused) VALUES (1, FALSE);",
        )
        .await
        .unwrap();

        Migration.up(&SchemaManager::new(&db)).await.unwrap();

        assert!(
            SchemaManager::new(&db)
                .has_column("grades", "is_absent")
                .await
                .unwrap()
        );
        let row = db
            .query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                "SELECT is_absent FROM grades WHERE id = 1".to_owned(),
            ))
            .await
            .unwrap()
            .unwrap();
        assert!(!row.try_get::<bool>("", "is_absent").unwrap());
    }
}
