use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FeeTemplates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(FeeTemplates::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(FeeTemplates::OrgId).uuid().not_null())
                    .col(ColumnDef::new(FeeTemplates::Name).string().not_null())
                    .col(ColumnDef::new(FeeTemplates::DefaultAmount).double().not_null())
                    .col(ColumnDef::new(FeeTemplates::RevenueAccountId).uuid().not_null())
                    .col(ColumnDef::new(FeeTemplates::ReceivableAccountId).uuid().not_null())
                    .col(ColumnDef::new(FeeTemplates::IsMandatory).boolean().not_null().default(true))
                    .col(ColumnDef::new(FeeTemplates::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(FeeTemplates::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeeTemplates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FeeTemplates {
    Table,
    Id,
    OrgId,
    Name,
    DefaultAmount,
    RevenueAccountId,
    ReceivableAccountId,
    IsMandatory,
    CreatedAt,
    UpdatedAt,
}
