use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClassSubjects::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ClassSubjects::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ClassSubjects::ClassId).uuid().not_null())
                    .col(ColumnDef::new(ClassSubjects::SubjectId).uuid().not_null())
                    .col(ColumnDef::new(ClassSubjects::PeriodId).uuid().not_null())
                    .col(ColumnDef::new(ClassSubjects::PrimaryTeacherId).uuid().not_null())
                    .col(ColumnDef::new(ClassSubjects::AssistantTeacherId).uuid().null())
                    .col(ColumnDef::new(ClassSubjects::SubstituteTeacherId).uuid().null())
                    .col(ColumnDef::new(ClassSubjects::GradeGroups).json().null())
                    .col(ColumnDef::new(ClassSubjects::WeeklyHours).integer().null())
                    .col(ColumnDef::new(ClassSubjects::TheoryHours).integer().null())
                    .col(ColumnDef::new(ClassSubjects::PracticalHours).integer().null())
                    .col(ColumnDef::new(ClassSubjects::AssessmentConfig).json().null())
                    .col(ColumnDef::new(ClassSubjects::SchemeId).uuid().null())
                    .col(ColumnDef::new(ClassSubjects::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(ClassSubjects::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ClassSubjects::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClassSubjects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ClassSubjects {
    Table,
    Id,
    ClassId,
    SubjectId,
    PeriodId,
    PrimaryTeacherId,
    AssistantTeacherId,
    SubstituteTeacherId,
    GradeGroups,
    WeeklyHours,
    TheoryHours,
    PracticalHours,
    AssessmentConfig,
    SchemeId,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
