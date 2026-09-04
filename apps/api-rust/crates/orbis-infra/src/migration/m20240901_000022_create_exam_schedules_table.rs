use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExamSchedules::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ExamSchedules::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ExamSchedules::ExamSessionId).uuid().not_null())
                    .col(ColumnDef::new(ExamSchedules::ClassSubjectId).uuid().not_null())
                    .col(ColumnDef::new(ExamSchedules::ExamDate).date().not_null())
                    .col(ColumnDef::new(ExamSchedules::StartTime).time().not_null())
                    .col(ColumnDef::new(ExamSchedules::EndTime).time().not_null())
                    .col(ColumnDef::new(ExamSchedules::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ExamSchedules::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExamSchedules::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ExamSchedules {
    Table,
    Id,
    ExamSessionId,
    ClassSubjectId,
    ExamDate,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}
