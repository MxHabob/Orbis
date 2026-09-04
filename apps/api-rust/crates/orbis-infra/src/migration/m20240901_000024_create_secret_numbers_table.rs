use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SecretNumbers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SecretNumbers::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(SecretNumbers::ExamScheduleId).uuid().not_null())
                    .col(ColumnDef::new(SecretNumbers::StudentId).uuid().not_null())
                    .col(ColumnDef::new(SecretNumbers::SecretNumber).string().not_null())
                    .col(ColumnDef::new(SecretNumbers::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(SecretNumbers::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SecretNumbers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SecretNumbers {
    Table,
    Id,
    ExamScheduleId,
    StudentId,
    SecretNumber,
    CreatedAt,
    UpdatedAt,
}
