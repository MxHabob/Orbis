use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReportTemplates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ReportTemplates::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ReportTemplates::OrgId).uuid().not_null())
                    .col(ColumnDef::new(ReportTemplates::Name).string().not_null())
                    .col(ColumnDef::new(ReportTemplates::Description).string())
                    .col(ColumnDef::new(ReportTemplates::TemplateType).string().not_null())
                    .col(ColumnDef::new(ReportTemplates::TemplateBody).text().not_null())
                    .col(ColumnDef::new(ReportTemplates::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(ReportTemplates::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ReportTemplates::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ReportJobs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ReportJobs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ReportJobs::OrgId).uuid().not_null())
                    .col(ColumnDef::new(ReportJobs::TemplateId).uuid().not_null())
                    .col(ColumnDef::new(ReportJobs::RequestedBy).uuid().not_null())
                    .col(ColumnDef::new(ReportJobs::Parameters).json().not_null())
                    .col(ColumnDef::new(ReportJobs::Status).string().not_null())
                    .col(ColumnDef::new(ReportJobs::FilePath).string())
                    .col(ColumnDef::new(ReportJobs::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ReportJobs::CompletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ReportJobs::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ReportTemplates::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum ReportTemplates {
    Table,
    Id,
    OrgId,
    Name,
    Description,
    TemplateType,
    TemplateBody,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ReportJobs {
    Table,
    Id,
    OrgId,
    TemplateId,
    RequestedBy,
    Parameters,
    Status,
    FilePath,
    CreatedAt,
    CompletedAt,
}
