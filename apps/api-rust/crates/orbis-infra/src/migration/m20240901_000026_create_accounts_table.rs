use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Accounts::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Accounts::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Accounts::Code).string().not_null())
                    .col(ColumnDef::new(Accounts::Name).string().not_null())
                    .col(ColumnDef::new(Accounts::AccountType).string().not_null())
                    .col(ColumnDef::new(Accounts::ParentId).uuid())
                    .col(ColumnDef::new(Accounts::Currency).string().not_null())
                    .col(ColumnDef::new(Accounts::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Accounts::Balances).json().not_null())
                    .col(ColumnDef::new(Accounts::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Accounts::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
    OrgId,
    Code,
    Name,
    AccountType,
    ParentId,
    Currency,
    IsActive,
    Balances,
    CreatedAt,
    UpdatedAt,
}
