use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Payments::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Payments::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Payments::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Payments::FeeId).uuid())
                    .col(ColumnDef::new(Payments::AccountId).uuid().not_null())
                    .col(ColumnDef::new(Payments::Amount).double().not_null())
                    .col(ColumnDef::new(Payments::Method).string().not_null())
                    .col(ColumnDef::new(Payments::Reference).string())
                    .col(ColumnDef::new(Payments::JournalEntryId).uuid())
                    .col(ColumnDef::new(Payments::PaidAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Payments::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Payments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
    OrgId,
    FeeId,
    AccountId,
    Amount,
    Method,
    Reference,
    JournalEntryId,
    PaidAt,
    CreatedAt,
}
