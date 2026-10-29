use crate::models::attendance;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_attendances_unique")
                    .table(attendance::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_attendances_lesson_unique")
                    .table(attendance::Entity)
                    .col(attendance::Column::StudentId)
                    .col(attendance::Column::ScheduledLessonId)
                    .col(attendance::Column::AttendanceDate)
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_attendances_lesson_unique")
                    .table(attendance::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_attendances_unique")
                    .table(attendance::Entity)
                    .col(attendance::Column::StudentId)
                    .col(attendance::Column::SectionSubjectId)
                    .col(attendance::Column::AttendanceDate)
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }
}
