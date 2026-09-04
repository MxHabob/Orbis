use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AcademicYears::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AcademicYears::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AcademicYears::OrgId).uuid().not_null())
                    .col(ColumnDef::new(AcademicYears::Name).string().not_null())
                    .col(ColumnDef::new(AcademicYears::NameAr).string().null())
                    .col(ColumnDef::new(AcademicYears::StartDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AcademicYears::EndDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AcademicYears::HijriStart).string().null())
                    .col(ColumnDef::new(AcademicYears::HijriEnd).string().null())
                    .col(ColumnDef::new(AcademicYears::Status).string().not_null())
                    .col(ColumnDef::new(AcademicYears::IsLocked).boolean().not_null().default(false))
                    .col(ColumnDef::new(AcademicYears::LockedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicYears::LockedBy).uuid().null())
                    .col(ColumnDef::new(AcademicYears::LockReason).string().null())
                    .col(ColumnDef::new(AcademicYears::CurrentPeriodId).uuid().null())
                    .col(ColumnDef::new(AcademicYears::ArchivedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(AcademicYears::ArchivedBy).uuid().null())
                    .col(ColumnDef::new(AcademicYears::Notes).string().null())
                    .col(ColumnDef::new(AcademicYears::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AcademicYears::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AcademicYears::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AcademicYears {
    Table,
    Id,
    OrgId,
    Name,
    NameAr,
    StartDate,
    EndDate,
    HijriStart,
    HijriEnd,
    Status,
    IsLocked,
    LockedAt,
    LockedBy,
    LockReason,
    CurrentPeriodId,
    ArchivedAt,
    ArchivedBy,
    Notes,
    CreatedAt,
    UpdatedAt,
}
