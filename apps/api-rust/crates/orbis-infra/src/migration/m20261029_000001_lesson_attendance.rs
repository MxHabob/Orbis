use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_attendances_unique")
            .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_attendances_lesson_unique \
             ON attendances (student_id, scheduled_lesson_id, attendance_date) \
             WHERE scheduled_lesson_id IS NOT NULL",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_attendances_legacy_unique \
             ON attendances (student_id, section_subject_id, attendance_date) \
             WHERE scheduled_lesson_id IS NULL",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_attendances_legacy_unique")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_attendances_lesson_unique")
            .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_attendances_unique \
             ON attendances (student_id, section_subject_id, attendance_date)",
        )
        .await?;
        Ok(())
    }
}
