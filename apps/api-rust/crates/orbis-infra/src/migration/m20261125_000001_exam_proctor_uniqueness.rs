use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::TransactionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_exam_proctors_assignment
             ON exam_proctors (exam_schedule_id, room_id, teacher_id)",
        )
        .await?;
        txn.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_exam_proctors_chief
             ON exam_proctors (exam_schedule_id, room_id) WHERE is_chief",
        )
        .await?;
        txn.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared("DROP INDEX IF EXISTS idx_exam_proctors_chief")
            .await?;
        txn.execute_unprepared("DROP INDEX IF EXISTS idx_exam_proctors_assignment")
            .await?;
        txn.commit().await
    }
}
