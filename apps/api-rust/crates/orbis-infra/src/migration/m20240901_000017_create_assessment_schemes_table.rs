use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AssessmentSchemes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AssessmentSchemes::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AssessmentSchemes::OrgId).uuid().not_null())
                    .col(ColumnDef::new(AssessmentSchemes::Name).string().not_null())
                    .col(ColumnDef::new(AssessmentSchemes::Description).string().null())
                    .col(ColumnDef::new(AssessmentSchemes::MaxScore).double().not_null())
                    .col(ColumnDef::new(AssessmentSchemes::Formula).json().null())
                    .col(ColumnDef::new(AssessmentSchemes::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AssessmentSchemes::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AssessmentSchemes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AssessmentSchemes {
    Table,
    Id,
    OrgId,
    Name,
    Description,
    MaxScore,
    Formula,
    CreatedAt,
    UpdatedAt,
}
