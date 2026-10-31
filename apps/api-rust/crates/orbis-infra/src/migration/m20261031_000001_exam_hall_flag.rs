use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager.has_column("rooms", "is_exam_hall").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Rooms::Table)
                        .add_column(
                            ColumnDef::new(Rooms::IsExamHall)
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
        if manager.has_column("rooms", "is_exam_hall").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Rooms::Table)
                        .drop_column(Rooms::IsExamHall)
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}

#[derive(Iden)]
enum Rooms {
    Table,
    IsExamHall,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migration_adds_exam_hall_flag() {
        let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
        crate::migration::Migrator::up(&db, None).await.unwrap();
        assert!(
            SchemaManager::new(&db)
                .has_column("rooms", "is_exam_hall")
                .await
                .unwrap()
        );
    }
}
