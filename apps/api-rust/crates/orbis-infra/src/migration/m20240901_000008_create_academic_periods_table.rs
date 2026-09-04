use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AcademicPeriods::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AcademicPeriods::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AcademicPeriods::OrgId).uuid().not_null())
                    .col(ColumnDef::new(AcademicPeriods::YearId).uuid().not_null())
                    .col(ColumnDef::new(AcademicPeriods::ParentId).uuid().null())
                    .col(ColumnDef::new(AcademicPeriods::Name).string().not_null())
                    .col(ColumnDef::new(AcademicPeriods::NameAr).string().null())
                    .col(ColumnDef::new(AcademicPeriods::Code).string().null())
                    .col(ColumnDef::new(AcademicPeriods::PeriodType).string().not_null())
                    .col(ColumnDef::new(AcademicPeriods::StartDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AcademicPeriods::EndDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AcademicPeriods::HijriStart).string().null())
                    .col(ColumnDef::new(AcademicPeriods::HijriEnd).string().null())
                    .col(ColumnDef::new(AcademicPeriods::EnrollmentStart).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicPeriods::EnrollmentEnd).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicPeriods::IsEnrollmentOpen).boolean().not_null().default(false))
                    .col(ColumnDef::new(AcademicPeriods::GradeEntryStart).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicPeriods::GradeEntryEnd).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicPeriods::IsGradeEntryOpen).boolean().not_null().default(false))
                    .col(ColumnDef::new(AcademicPeriods::ExamStart).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicPeriods::ExamEnd).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicPeriods::ScaleId).uuid().null())
                    .col(ColumnDef::new(AcademicPeriods::PassFailPolicyId).uuid().null())
                    .col(ColumnDef::new(AcademicPeriods::Status).string().not_null())
                    .col(ColumnDef::new(AcademicPeriods::IsLocked).boolean().not_null().default(false))
                    .col(ColumnDef::new(AcademicPeriods::LockedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicPeriods::LockedBy).uuid().null())
                    .col(ColumnDef::new(AcademicPeriods::SortOrder).integer().null())
                    .col(ColumnDef::new(AcademicPeriods::Metadata).json().null())
                    .col(ColumnDef::new(AcademicPeriods::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AcademicPeriods::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AcademicPeriods::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AcademicPeriods {
    Table,
    Id,
    OrgId,
    YearId,
    ParentId,
    Name,
    NameAr,
    Code,
    PeriodType,
    StartDate,
    EndDate,
    HijriStart,
    HijriEnd,
    EnrollmentStart,
    EnrollmentEnd,
    IsEnrollmentOpen,
    GradeEntryStart,
    GradeEntryEnd,
    IsGradeEntryOpen,
    ExamStart,
    ExamEnd,
    ScaleId,
    PassFailPolicyId,
    Status,
    IsLocked,
    LockedAt,
    LockedBy,
    SortOrder,
    Metadata,
    CreatedAt,
    UpdatedAt,
}
