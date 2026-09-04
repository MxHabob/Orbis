use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JournalEntries::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(JournalEntries::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(JournalEntries::OrgId).uuid().not_null())
                    .col(ColumnDef::new(JournalEntries::TransactionRef).string().not_null())
                    .col(ColumnDef::new(JournalEntries::AccountingDate).date().not_null())
                    .col(ColumnDef::new(JournalEntries::Amount).double().not_null())
                    .col(ColumnDef::new(JournalEntries::CurrencyId).string().not_null())
                    .col(ColumnDef::new(JournalEntries::ExchangeRate).double().not_null().default(1.0))
                    .col(ColumnDef::new(JournalEntries::BaseAmount).double().not_null())
                    .col(ColumnDef::new(JournalEntries::DebitAccountId).uuid().not_null())
                    .col(ColumnDef::new(JournalEntries::CreditAccountId).uuid().not_null())
                    .col(ColumnDef::new(JournalEntries::Description).string().not_null())
                    .col(ColumnDef::new(JournalEntries::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(JournalEntries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum JournalEntries {
    Table,
    Id,
    OrgId,
    TransactionRef,
    AccountingDate,
    Amount,
    CurrencyId,
    ExchangeRate,
    BaseAmount,
    DebitAccountId,
    CreditAccountId,
    Description,
    CreatedAt,
}
