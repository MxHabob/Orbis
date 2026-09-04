use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IntegrationConfigs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(IntegrationConfigs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(IntegrationConfigs::OrgId).uuid().not_null())
                    .col(ColumnDef::new(IntegrationConfigs::ProviderName).string().not_null())
                    .col(ColumnDef::new(IntegrationConfigs::Category).string().not_null())
                    .col(ColumnDef::new(IntegrationConfigs::IsEnabled).boolean().not_null().default(true))
                    .col(ColumnDef::new(IntegrationConfigs::Env).string().not_null())
                    .col(ColumnDef::new(IntegrationConfigs::Settings).json())
                    .col(ColumnDef::new(IntegrationConfigs::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(IntegrationConfigs::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IntegrationSecrets::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(IntegrationSecrets::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(IntegrationSecrets::ConfigId).uuid().not_null())
                    .col(ColumnDef::new(IntegrationSecrets::SecretKey).string().not_null())
                    .col(ColumnDef::new(IntegrationSecrets::KeyName).string().not_null())
                    .col(ColumnDef::new(IntegrationSecrets::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(IntegrationSecrets::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IntegrationSecrets::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(IntegrationConfigs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IntegrationConfigs {
    Table,
    Id,
    OrgId,
    ProviderName,
    Category,
    IsEnabled,
    Env,
    Settings,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum IntegrationSecrets {
    Table,
    Id,
    ConfigId,
    SecretKey,
    KeyName,
    CreatedAt,
    UpdatedAt,
}
