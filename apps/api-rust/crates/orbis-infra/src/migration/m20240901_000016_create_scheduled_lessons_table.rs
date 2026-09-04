use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ScheduledLessons::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ScheduledLessons::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ScheduledLessons::ClassSubjectId).uuid().not_null())
                    .col(ColumnDef::new(ScheduledLessons::RoomId).uuid().null())
                    .col(ColumnDef::new(ScheduledLessons::TimeSlotId).uuid().not_null())
                    .col(ColumnDef::new(ScheduledLessons::TeacherId).uuid().null())
                    .col(ColumnDef::new(ScheduledLessons::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ScheduledLessons::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ScheduledLessons::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ScheduledLessons {
    Table,
    Id,
    ClassSubjectId,
    RoomId,
    TimeSlotId,
    TeacherId,
    CreatedAt,
    UpdatedAt,
}
