use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PromotionJobs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PromotionJobs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(PromotionJobs::OrgId).uuid().not_null())
                    .col(ColumnDef::new(PromotionJobs::SourcePeriodId).uuid().not_null())
                    .col(ColumnDef::new(PromotionJobs::TargetPeriodId).uuid().null())
                    .col(ColumnDef::new(PromotionJobs::Name).string().not_null())
                    .col(ColumnDef::new(PromotionJobs::Status).string().not_null().default("draft"))
                    .col(ColumnDef::new(PromotionJobs::TotalStudents).integer().not_null().default(0))
                    .col(ColumnDef::new(PromotionJobs::PromotedCount).integer().not_null().default(0))
                    .col(ColumnDef::new(PromotionJobs::RetainedCount).integer().not_null().default(0))
                    .col(ColumnDef::new(PromotionJobs::GraduatedCount).integer().not_null().default(0))
                    .col(ColumnDef::new(PromotionJobs::ExcludedCount).integer().not_null().default(0))
                    .col(ColumnDef::new(PromotionJobs::ExecutedBy).uuid().null())
                    .col(ColumnDef::new(PromotionJobs::ExecutedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(PromotionJobs::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(PromotionJobs::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PromotionDetails::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PromotionDetails::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(PromotionDetails::JobId).uuid().not_null())
                    .col(ColumnDef::new(PromotionDetails::StudentId).uuid().not_null())
                    .col(ColumnDef::new(PromotionDetails::EnrollmentId).uuid().not_null())
                    .col(ColumnDef::new(PromotionDetails::SourceClassId).uuid().null())
                    .col(ColumnDef::new(PromotionDetails::TargetClassId).uuid().null())
                    .col(ColumnDef::new(PromotionDetails::Decision).string().not_null())
                    .col(ColumnDef::new(PromotionDetails::Reason).string().not_null())
                    .col(ColumnDef::new(PromotionDetails::ManualOverride).boolean().not_null().default(false))
                    .col(ColumnDef::new(PromotionDetails::OverrideBy).uuid().null())
                    .col(ColumnDef::new(PromotionDetails::OverrideReason).string().null())
                    .col(ColumnDef::new(PromotionDetails::FinalGpa).decimal_len(10, 4).null())
                    .col(ColumnDef::new(PromotionDetails::AttendanceRate).decimal_len(5, 2).null())
                    .col(ColumnDef::new(PromotionDetails::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_promotion_detail_job")
                            .from(PromotionDetails::Table, PromotionDetails::JobId)
                            .to(PromotionJobs::Table, PromotionJobs::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_prom_job_org_status")
                    .table(PromotionJobs::Table)
                    .col(PromotionJobs::OrgId)
                    .col(PromotionJobs::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_prom_detail_student")
                    .table(PromotionDetails::Table)
                    .col(PromotionDetails::JobId)
                    .col(PromotionDetails::StudentId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(PromotionDetails::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(PromotionJobs::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum PromotionJobs {
    Table,
    Id,
    OrgId,
    SourcePeriodId,
    TargetPeriodId,
    Name,
    Status,
    TotalStudents,
    PromotedCount,
    RetainedCount,
    GraduatedCount,
    ExcludedCount,
    ExecutedBy,
    ExecutedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum PromotionDetails {
    Table,
    Id,
    JobId,
    StudentId,
    EnrollmentId,
    SourceClassId,
    TargetClassId,
    Decision,
    Reason,
    ManualOverride,
    OverrideBy,
    OverrideReason,
    FinalGpa,
    AttendanceRate,
    CreatedAt,
}
