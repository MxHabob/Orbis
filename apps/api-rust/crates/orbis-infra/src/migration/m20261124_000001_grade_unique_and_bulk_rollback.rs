use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DELETE FROM grades
                 WHERE id IN (
                   SELECT id FROM (
                     SELECT id,
                            ROW_NUMBER() OVER (
                              PARTITION BY org_id, student_id, period_id,
                                           section_subject_id, component_id
                              ORDER BY created_at, id
                            ) AS duplicate_rank
                     FROM grades
                     WHERE component_id IS NOT NULL
                   ) ranked
                   WHERE duplicate_rank > 1
                 )",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE students
                 SET is_deleted = TRUE,
                     deleted_at = COALESCE(deleted_at, CURRENT_TIMESTAMP)
                 WHERE id IN (
                   SELECT id FROM (
                     SELECT id,
                            ROW_NUMBER() OVER (
                              PARTITION BY org_id, person_id
                              ORDER BY admitted_at, created_at, id
                            ) AS duplicate_rank
                     FROM students
                   ) ranked
                   WHERE duplicate_rank > 1
                 )",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE UNIQUE INDEX IF NOT EXISTS idx_students_org_person
                 ON students (org_id, person_id)
                 WHERE is_deleted = FALSE",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "DELETE FROM grades
                 WHERE id IN (
                   SELECT id FROM (
                     SELECT id,
                            ROW_NUMBER() OVER (
                              PARTITION BY org_id, student_id, period_id,
                                           section_subject_id
                              ORDER BY created_at, id
                            ) AS duplicate_rank
                     FROM grades
                     WHERE component_id IS NULL
                   ) ranked
                   WHERE duplicate_rank > 1
                 )",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE UNIQUE INDEX IF NOT EXISTS idx_grades_unique_component
                 ON grades (org_id, student_id, period_id, section_subject_id, component_id)
                 WHERE component_id IS NOT NULL",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE UNIQUE INDEX IF NOT EXISTS idx_grades_unique_no_component
                 ON grades (org_id, student_id, period_id, section_subject_id)
                 WHERE component_id IS NULL",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP INDEX IF EXISTS idx_students_org_person")
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP INDEX IF EXISTS idx_grades_unique_component")
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP INDEX IF EXISTS idx_grades_unique_no_component")
            .await?;
        Ok(())
    }
}
