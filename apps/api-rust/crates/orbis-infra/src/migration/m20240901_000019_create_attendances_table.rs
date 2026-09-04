use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Attendances::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Attendances::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Attendances::StudentId).uuid().not_null())
                    .col(ColumnDef::new(Attendances::ClassId).uuid().not_null())
                    .col(ColumnDef::new(Attendances::Date).date().not_null())
                    .col(ColumnDef::new(Attendances::Status).string().not_null())
                    .col(ColumnDef::new(Attendances::Notes).string().null())
                    .col(ColumnDef::new(Attendances::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Attendances::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Attendances::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Attendances {
    Table,
    Id,
    StudentId,
    ClassId,
    Date,
    Status,
    Notes,
    CreatedAt,
    UpdatedAt,
}
