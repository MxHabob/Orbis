use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AcademicOverrideRequests::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AcademicOverrideRequests::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AcademicOverrideRequests::OrgId).uuid().not_null())
                    .col(ColumnDef::new(AcademicOverrideRequests::OverrideType).string().not_null())
                    .col(ColumnDef::new(AcademicOverrideRequests::RiskLevel).string().not_null())
                    .col(ColumnDef::new(AcademicOverrideRequests::EntityType).string().not_null())
                    .col(ColumnDef::new(AcademicOverrideRequests::EntityId).uuid().not_null())
                    .col(ColumnDef::new(AcademicOverrideRequests::FieldName).string().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::OldValue).string().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::NewValue).string().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::Reason).string().not_null())
                    .col(ColumnDef::new(AcademicOverrideRequests::RequestedBy).uuid().not_null())
                    .col(ColumnDef::new(AcademicOverrideRequests::RequestedByName).string().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::Status).string().not_null().default("pending"))
                    .col(ColumnDef::new(AcademicOverrideRequests::CurrentApproverRole).string().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::ApprovalChain).json().not_null())
                    .col(ColumnDef::new(AcademicOverrideRequests::ResolvedBy).uuid().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::ResolvedByName).string().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::ResolvedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::ResolutionNotes).string().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::ExpiresAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::Metadata).json().null())
                    .col(ColumnDef::new(AcademicOverrideRequests::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AcademicOverrideRequests::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_override_org")
                            .from(AcademicOverrideRequests::Table, AcademicOverrideRequests::OrgId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_override_org_status")
                    .table(AcademicOverrideRequests::Table)
                    .col(AcademicOverrideRequests::OrgId)
                    .col(AcademicOverrideRequests::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_override_entity")
                    .table(AcademicOverrideRequests::Table)
                    .col(AcademicOverrideRequests::OrgId)
                    .col(AcademicOverrideRequests::EntityType)
                    .col(AcademicOverrideRequests::EntityId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AcademicOverrideRequests::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum AcademicOverrideRequests {
    Table,
    Id,
    OrgId,
    OverrideType,
    RiskLevel,
    EntityType,
    EntityId,
    FieldName,
    OldValue,
    NewValue,
    Reason,
    RequestedBy,
    RequestedByName,
    Status,
    CurrentApproverRole,
    ApprovalChain,
    ResolvedBy,
    ResolvedByName,
    ResolvedAt,
    ResolutionNotes,
    ExpiresAt,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}
