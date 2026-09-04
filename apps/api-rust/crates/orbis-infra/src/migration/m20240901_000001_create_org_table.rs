use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organizations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Organizations::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Organizations::Name).string().not_null())
                    .col(ColumnDef::new(Organizations::NameAr).string().null())
                    .col(ColumnDef::new(Organizations::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(Organizations::Code).string().null().unique_key())
                    .col(ColumnDef::new(Organizations::Timezone).string().not_null())
                    .col(ColumnDef::new(Organizations::Locale).string().not_null())
                    .col(ColumnDef::new(Organizations::DateFormat).string().null())
                    .col(ColumnDef::new(Organizations::HijriEnabled).boolean().not_null().default(true))
                    .col(ColumnDef::new(Organizations::MaxUsers).integer().not_null().default(1000))
                    .col(ColumnDef::new(Organizations::MaxStudents).integer().not_null().default(5000))
                    .col(ColumnDef::new(Organizations::MaxStorageGb).integer().not_null().default(100))
                    .col(ColumnDef::new(Organizations::Settings).json().null())
                    .col(ColumnDef::new(Organizations::Features).json().null())
                    .col(ColumnDef::new(Organizations::Logo).string().null())
                    .col(ColumnDef::new(Organizations::Favicon).string().null())
                    .col(ColumnDef::new(Organizations::PrimaryColor).string().null())
                    .col(ColumnDef::new(Organizations::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Organizations::IsVerified).boolean().not_null().default(false))
                    .col(ColumnDef::new(Organizations::IsDeleted).boolean().not_null().default(false))
                    .col(ColumnDef::new(Organizations::DeletedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Organizations::DeletedBy).uuid().null())
                    .col(ColumnDef::new(Organizations::CurrentPeriodId).string().null())
                    .col(ColumnDef::new(Organizations::Metadata).json().null())
                    .col(ColumnDef::new(Organizations::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Organizations::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organizations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Organizations {
    Table,
    Id,
    Name,
    NameAr,
    Slug,
    Code,
    Timezone,
    Locale,
    DateFormat,
    HijriEnabled,
    MaxUsers,
    MaxStudents,
    MaxStorageGb,
    Settings,
    Features,
    Logo,
    Favicon,
    PrimaryColor,
    IsActive,
    IsVerified,
    IsDeleted,
    DeletedAt,
    DeletedBy,
    CurrentPeriodId,
    Metadata,
    CreatedAt,
    UpdatedAt,
}
