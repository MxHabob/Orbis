use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager.has_column("exam_sessions", "name_ar").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(ExamSessions::Table)
                        .add_column(ColumnDef::new(ExamSessions::NameAr).string().null())
                        .to_owned(),
                )
                .await?;
        }

        if !manager.has_column("exam_sessions", "session_type").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(ExamSessions::Table)
                        .add_column(
                            ColumnDef::new(ExamSessions::SessionType)
                                .string()
                                .not_null()
                                .default("final"),
                        )
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_column("exam_sessions", "session_type").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(ExamSessions::Table)
                        .drop_column(ExamSessions::SessionType)
                        .to_owned(),
                )
                .await?;
        }

        if manager.has_column("exam_sessions", "name_ar").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(ExamSessions::Table)
                        .drop_column(ExamSessions::NameAr)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }
}

#[derive(Iden)]
enum ExamSessions {
    Table,
    NameAr,
    SessionType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migration_adds_exam_session_details() {
        let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
        crate::migration::Migrator::up(&db, None).await.unwrap();

        let manager = SchemaManager::new(&db);
        assert!(
            manager
                .has_column("exam_sessions", "name_ar")
                .await
                .unwrap()
        );
        assert!(
            manager
                .has_column("exam_sessions", "session_type")
                .await
                .unwrap()
        );
    }
}
