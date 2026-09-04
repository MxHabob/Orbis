use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Enrollments::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Enrollments::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Enrollments::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Enrollments::PeriodId).uuid().not_null())
                    .col(ColumnDef::new(Enrollments::StudentId).uuid().not_null())
                    .col(ColumnDef::new(Enrollments::ClassId).uuid().null())
                    .col(ColumnDef::new(Enrollments::StudentNumber).string().not_null().unique_key())
                    .col(ColumnDef::new(Enrollments::Status).string().not_null())
                    .col(ColumnDef::new(Enrollments::EnrollmentDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Enrollments::EnrollmentType).string().not_null())
                    .col(ColumnDef::new(Enrollments::WithdrawalDate).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Enrollments::WithdrawalReason).string().null())
                    .col(ColumnDef::new(Enrollments::Meta).json().null())
                    .col(ColumnDef::new(Enrollments::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Enrollments::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Enrollments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Enrollments {
    Table,
    Id,
    OrgId,
    PeriodId,
    StudentId,
    ClassId,
    StudentNumber,
    Status,
    EnrollmentDate,
    EnrollmentType,
    WithdrawalDate,
    WithdrawalReason,
    Meta,
    CreatedAt,
    UpdatedAt,
}
