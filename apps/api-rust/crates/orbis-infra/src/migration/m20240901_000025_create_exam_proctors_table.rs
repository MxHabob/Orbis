use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExamProctors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ExamProctors::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ExamProctors::ExamScheduleId).uuid().not_null())
                    .col(ColumnDef::new(ExamProctors::RoomId).uuid().not_null())
                    .col(ColumnDef::new(ExamProctors::TeacherId).uuid().not_null())
                    .col(ColumnDef::new(ExamProctors::IsChief).boolean().not_null().default(false))
                    .col(ColumnDef::new(ExamProctors::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ExamProctors::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExamProctors::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ExamProctors {
    Table,
    Id,
    ExamScheduleId,
    RoomId,
    TeacherId,
    IsChief,
    CreatedAt,
    UpdatedAt,
}
