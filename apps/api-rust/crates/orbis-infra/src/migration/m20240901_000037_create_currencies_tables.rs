use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Currencies Table
        manager
            .create_table(
                Table::create()
                    .table(Currencies::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Currencies::Id).string().not_null().primary_key()) // e.g., 'USD', 'SAR'
                    .col(ColumnDef::new(Currencies::Name).string().not_null())
                    .col(ColumnDef::new(Currencies::NameAr).string().null())
                    .col(ColumnDef::new(Currencies::Symbol).string().not_null())
                    .col(ColumnDef::new(Currencies::DecimalPlaces).integer().not_null().default(2))
                    .col(ColumnDef::new(Currencies::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Currencies::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // 2. Exchange Rates Table
        manager
            .create_table(
                Table::create()
                    .table(ExchangeRates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ExchangeRates::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ExchangeRates::OrgId).uuid().not_null())
                    .col(ColumnDef::new(ExchangeRates::BaseCurrencyId).string().not_null())
                    .col(ColumnDef::new(ExchangeRates::TargetCurrencyId).string().not_null())
                    .col(ColumnDef::new(ExchangeRates::Rate).double().not_null())
                    .col(ColumnDef::new(ExchangeRates::EffectiveDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ExchangeRates::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ExchangeRates::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Currencies::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Currencies {
    Table,
    Id, // The ISO code (e.g., 'USD', 'SAR')
    Name,
    NameAr,
    Symbol,
    DecimalPlaces,
    IsActive,
    CreatedAt,
}

#[derive(DeriveIden)]
enum ExchangeRates {
    Table,
    Id,
    OrgId,
    BaseCurrencyId,
    TargetCurrencyId,
    Rate,
    EffectiveDate,
    CreatedAt,
}
