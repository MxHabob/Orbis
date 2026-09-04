use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop old tables first (clean slate, no backward compat needed)
        manager
            .drop_table(Table::drop().table(Alias::new("report_jobs")).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Alias::new("report_templates")).if_exists().to_owned())
            .await?;

        // ── report_definitions ─────────────────────────────────────
        manager
            .create_table(
                Table::create()
                    .table(ReportDefinitions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ReportDefinitions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ReportDefinitions::OrgId).uuid().not_null())
                    .col(ColumnDef::new(ReportDefinitions::Code).string().not_null())
                    .col(ColumnDef::new(ReportDefinitions::Name).string().not_null())
                    .col(ColumnDef::new(ReportDefinitions::NameAr).string().not_null())
                    .col(ColumnDef::new(ReportDefinitions::Domain).string().not_null())
                    .col(ColumnDef::new(ReportDefinitions::Category).string().not_null())
                    .col(ColumnDef::new(ReportDefinitions::Description).string())
                    .col(ColumnDef::new(ReportDefinitions::DescriptionAr).string())
                    .col(ColumnDef::new(ReportDefinitions::ParametersJson).text().not_null().default("[]"))
                    .col(ColumnDef::new(ReportDefinitions::RequiredPermissions).text().not_null().default("[]"))
                    .col(ColumnDef::new(ReportDefinitions::SupportedFormats).text().not_null().default("[]"))
                    .col(ColumnDef::new(ReportDefinitions::DataProviderKey).string().not_null())
                    .col(ColumnDef::new(ReportDefinitions::IsSystem).boolean().not_null().default(false))
                    .col(ColumnDef::new(ReportDefinitions::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(ReportDefinitions::SortOrder).integer().not_null().default(0))
                    .col(ColumnDef::new(ReportDefinitions::Version).integer().not_null().default(1))
                    .col(ColumnDef::new(ReportDefinitions::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ReportDefinitions::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Unique index on (org_id, code)
        manager
            .create_index(
                Index::create()
                    .name("idx_report_definitions_org_code")
                    .table(ReportDefinitions::Table)
                    .col(ReportDefinitions::OrgId)
                    .col(ReportDefinitions::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // ── report_templates ───────────────────────────────────────
        manager
            .create_table(
                Table::create()
                    .table(ReportTemplates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ReportTemplates::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ReportTemplates::OrgId).uuid().not_null())
                    .col(ColumnDef::new(ReportTemplates::Code).string())
                    .col(ColumnDef::new(ReportTemplates::Name).string().not_null())
                    .col(ColumnDef::new(ReportTemplates::NameAr).string())
                    .col(ColumnDef::new(ReportTemplates::Description).string())
                    .col(ColumnDef::new(ReportTemplates::DescriptionAr).string())
                    .col(ColumnDef::new(ReportTemplates::DocumentType).string().not_null())
                    .col(ColumnDef::new(ReportTemplates::EntityType).string().not_null())
                    .col(ColumnDef::new(ReportTemplates::MasterTemplateId).uuid())
                    .col(ColumnDef::new(ReportTemplates::Status).string().not_null().default("draft"))
                    .col(ColumnDef::new(ReportTemplates::Version).integer().not_null().default(1))
                    .col(ColumnDef::new(ReportTemplates::PublishedVersion).integer())
                    .col(ColumnDef::new(ReportTemplates::PublishedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ReportTemplates::PublishedBy).uuid())
                    .col(ColumnDef::new(ReportTemplates::PageSize).string().not_null().default("A4"))
                    .col(ColumnDef::new(ReportTemplates::PageWidthMm).double())
                    .col(ColumnDef::new(ReportTemplates::PageHeightMm).double())
                    .col(ColumnDef::new(ReportTemplates::Orientation).string().not_null().default("portrait"))
                    .col(ColumnDef::new(ReportTemplates::MarginsJson).text().not_null().default(r#"{"top":10,"bottom":10,"left":10,"right":10}"#))
                    .col(ColumnDef::new(ReportTemplates::DocumentJson).text().not_null().default("{}"))
                    .col(ColumnDef::new(ReportTemplates::Locale).string().not_null().default("ar"))
                    .col(ColumnDef::new(ReportTemplates::Tags).string())
                    .col(ColumnDef::new(ReportTemplates::IsSystem).boolean().not_null().default(false))
                    .col(ColumnDef::new(ReportTemplates::IsDefault).boolean().not_null().default(false))
                    .col(ColumnDef::new(ReportTemplates::CreatedBy).uuid().not_null())
                    .col(ColumnDef::new(ReportTemplates::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ReportTemplates::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ReportTemplates::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // ── report_template_versions ───────────────────────────────
        manager
            .create_table(
                Table::create()
                    .table(ReportTemplateVersions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ReportTemplateVersions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ReportTemplateVersions::TemplateId).uuid().not_null())
                    .col(ColumnDef::new(ReportTemplateVersions::Version).integer().not_null())
                    .col(ColumnDef::new(ReportTemplateVersions::DocumentJson).text().not_null())
                    .col(ColumnDef::new(ReportTemplateVersions::PageSize).string())
                    .col(ColumnDef::new(ReportTemplateVersions::PageWidthMm).double())
                    .col(ColumnDef::new(ReportTemplateVersions::PageHeightMm).double())
                    .col(ColumnDef::new(ReportTemplateVersions::Orientation).string())
                    .col(ColumnDef::new(ReportTemplateVersions::MarginsJson).text())
                    .col(ColumnDef::new(ReportTemplateVersions::ChangeSummary).string())
                    .col(ColumnDef::new(ReportTemplateVersions::CreatedBy).uuid().not_null())
                    .col(ColumnDef::new(ReportTemplateVersions::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // ── report_template_bindings ───────────────────────────────
        manager
            .create_table(
                Table::create()
                    .table(ReportTemplateBindings::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ReportTemplateBindings::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ReportTemplateBindings::ReportDefinitionId).uuid().not_null())
                    .col(ColumnDef::new(ReportTemplateBindings::TemplateId).uuid().not_null())
                    .col(ColumnDef::new(ReportTemplateBindings::Locale).string().not_null())
                    .col(ColumnDef::new(ReportTemplateBindings::Format).string().not_null())
                    .col(ColumnDef::new(ReportTemplateBindings::IsDefault).boolean().not_null().default(true))
                    .col(ColumnDef::new(ReportTemplateBindings::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // ── report_jobs ────────────────────────────────────────────
        manager
            .create_table(
                Table::create()
                    .table(ReportJobs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ReportJobs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ReportJobs::OrgId).uuid().not_null())
                    .col(ColumnDef::new(ReportJobs::ReportDefinitionId).uuid().not_null())
                    .col(ColumnDef::new(ReportJobs::TemplateId).uuid())
                    .col(ColumnDef::new(ReportJobs::RequestedBy).uuid().not_null())
                    .col(ColumnDef::new(ReportJobs::ReportCode).string().not_null())
                    .col(ColumnDef::new(ReportJobs::Parameters).json().not_null())
                    .col(ColumnDef::new(ReportJobs::OutputFormat).string().not_null())
                    .col(ColumnDef::new(ReportJobs::Locale).string().not_null().default("ar"))
                    .col(ColumnDef::new(ReportJobs::Status).string().not_null().default("pending"))
                    .col(ColumnDef::new(ReportJobs::OutputPath).string())
                    .col(ColumnDef::new(ReportJobs::OutputSizeBytes).big_integer())
                    .col(ColumnDef::new(ReportJobs::ErrorMessage).text())
                    .col(ColumnDef::new(ReportJobs::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ReportJobs::StartedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ReportJobs::CompletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // ── report_audit_log ───────────────────────────────────────
        manager
            .create_table(
                Table::create()
                    .table(ReportAuditLog::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ReportAuditLog::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ReportAuditLog::OrgId).uuid().not_null())
                    .col(ColumnDef::new(ReportAuditLog::ActorId).uuid().not_null())
                    .col(ColumnDef::new(ReportAuditLog::Action).string().not_null())
                    .col(ColumnDef::new(ReportAuditLog::EntityType).string().not_null())
                    .col(ColumnDef::new(ReportAuditLog::EntityId).uuid().not_null())
                    .col(ColumnDef::new(ReportAuditLog::Details).json())
                    .col(ColumnDef::new(ReportAuditLog::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ReportAuditLog::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(ReportJobs::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(ReportTemplateBindings::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(ReportTemplateVersions::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(ReportTemplates::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(ReportDefinitions::Table).if_exists().to_owned()).await?;
        Ok(())
    }
}

// ── Identifier Enums ────────────────────────────────────────────

#[derive(DeriveIden)]
enum ReportDefinitions {
    Table,
    Id, OrgId, Code, Name, NameAr, Domain, Category,
    Description, DescriptionAr, ParametersJson,
    RequiredPermissions, SupportedFormats, DataProviderKey,
    IsSystem, IsActive, SortOrder, Version,
    CreatedAt, UpdatedAt,
}

#[derive(DeriveIden)]
enum ReportTemplates {
    Table,
    Id, OrgId, Code, Name, NameAr, Description, DescriptionAr,
    DocumentType, EntityType, MasterTemplateId,
    Status, Version, PublishedVersion, PublishedAt, PublishedBy,
    PageSize, PageWidthMm, PageHeightMm, Orientation, MarginsJson,
    DocumentJson, Locale, Tags, IsSystem, IsDefault,
    CreatedBy, DeletedAt, CreatedAt, UpdatedAt,
}

#[derive(DeriveIden)]
enum ReportTemplateVersions {
    Table,
    Id, TemplateId, Version, DocumentJson,
    PageSize, PageWidthMm, PageHeightMm, Orientation, MarginsJson,
    ChangeSummary, CreatedBy, CreatedAt,
}

#[derive(DeriveIden)]
enum ReportTemplateBindings {
    Table,
    Id, ReportDefinitionId, TemplateId, Locale, Format,
    IsDefault, CreatedAt,
}

#[derive(DeriveIden)]
enum ReportJobs {
    Table,
    Id, OrgId, ReportDefinitionId, TemplateId, RequestedBy,
    ReportCode, Parameters, OutputFormat, Locale, Status,
    OutputPath, OutputSizeBytes, ErrorMessage,
    CreatedAt, StartedAt, CompletedAt,
}

#[derive(DeriveIden)]
enum ReportAuditLog {
    Table,
    Id, OrgId, ActorId, Action, EntityType, EntityId,
    Details, CreatedAt,
}
