use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PeriodTemplates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PeriodTemplates::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(PeriodTemplates::OrgId).uuid().not_null())
                    .col(ColumnDef::new(PeriodTemplates::Name).string().not_null())
                    .col(ColumnDef::new(PeriodTemplates::NameAr).string().null())
                    .col(ColumnDef::new(PeriodTemplates::InstitutionType).string().not_null())
                    .col(ColumnDef::new(PeriodTemplates::Structure).json().not_null())
                    .col(ColumnDef::new(PeriodTemplates::IsDefault).boolean().not_null().default(false))
                    .col(ColumnDef::new(PeriodTemplates::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(PeriodTemplates::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PeriodTemplates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PeriodTemplates {
    Table,
    Id,
    OrgId,
    Name,
    NameAr,
    InstitutionType,
    Structure,
    IsDefault,
    CreatedAt,
    UpdatedAt,
}
