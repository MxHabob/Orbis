use sea_orm_migration::{prelude::*, schema::*};
use super::m20240901_000001_create_org_table::Organizations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::Id))
                    .col(uuid(Users::PersonId))
                    .col(uuid_null(Users::OrgId))
                    .col(string_null(Users::Email).unique_key())
                    .col(boolean(Users::EmailVerified).default(false))
                    .col(string_null(Users::Phone))
                    .col(boolean(Users::PhoneVerified).default(false))
                    .col(string_null(Users::PasswordHash))
                    .col(boolean(Users::MfaEnabled).default(false))
                    .col(string(Users::FullName))
                    .col(string(Users::Status).default("active"))
                    .col(integer(Users::LoginCount).default(0))
                    .col(integer(Users::FailedLogins).default(0))
                    .col(timestamp_with_time_zone_null(Users::LockedUntil))
                    .col(boolean(Users::IsDeleted).default(false))
                    .col(timestamp_with_time_zone(Users::CreatedAt))
                    .col(timestamp_with_time_zone(Users::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-org_id")
                            .from(Users::Table, Users::OrgId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    PersonId,
    OrgId,
    Email,
    EmailVerified,
    Phone,
    PhoneVerified,
    PasswordHash,
    MfaEnabled,
    FullName,
    Status,
    LoginCount,
    FailedLogins,
    LockedUntil,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}
