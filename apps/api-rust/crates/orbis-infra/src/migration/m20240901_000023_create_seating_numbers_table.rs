use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SeatingNumbers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SeatingNumbers::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(SeatingNumbers::ExamSessionId).uuid().not_null())
                    .col(ColumnDef::new(SeatingNumbers::StudentId).uuid().not_null())
                    .col(ColumnDef::new(SeatingNumbers::SeatingNumber).string().not_null())
                    .col(ColumnDef::new(SeatingNumbers::RoomId).uuid().null())
                    .col(ColumnDef::new(SeatingNumbers::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(SeatingNumbers::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SeatingNumbers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SeatingNumbers {
    Table,
    Id,
    ExamSessionId,
    StudentId,
    SeatingNumber,
    RoomId,
    CreatedAt,
    UpdatedAt,
}
