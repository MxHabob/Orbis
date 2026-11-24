use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager.has_column("grades", "component_code").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Grades::Table)
                        .add_column(ColumnDef::new(Grades::ComponentCode).string().null())
                        .to_owned(),
                )
                .await?;
        }

        manager
            .get_connection()
            .execute_unprepared(
                "DROP INDEX IF EXISTS idx_grades_unique_no_component;
                 CREATE UNIQUE INDEX IF NOT EXISTS idx_grades_unique_no_component
                 ON grades (org_id, student_id, period_id, section_subject_id)
                 WHERE component_id IS NULL AND component_code IS NULL;
                 CREATE UNIQUE INDEX IF NOT EXISTS idx_grades_unique_component_code
                 ON grades (org_id, student_id, period_id, section_subject_id, component_code)
                 WHERE component_code IS NOT NULL",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP INDEX IF EXISTS idx_grades_unique_component_code;
                 DROP INDEX IF EXISTS idx_grades_unique_no_component;
                 CREATE UNIQUE INDEX IF NOT EXISTS idx_grades_unique_no_component
                 ON grades (org_id, student_id, period_id, section_subject_id)
                 WHERE component_id IS NULL",
            )
            .await?;

        if manager.has_column("grades", "component_code").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Grades::Table)
                        .drop_column(Grades::ComponentCode)
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
    ComponentCode,
}
