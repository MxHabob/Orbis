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
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(uuid(User::PersonId))
                    .col(uuid_null(User::PartyId))
                    .col(uuid_null(User::OrgId))
                    .col(string_null(User::Email).unique_key())
                    .col(boolean(User::EmailVerified).default(false))
                    .col(string_null(User::Phone))
                    .col(boolean(User::PhoneVerified).default(false))
                    .col(string_null(User::PasswordHash))
                    .col(boolean(User::MfaEnabled).default(false))
                    .col(string(User::FullName))
                    .col(string(User::PrimaryRole))
                    .col(string(User::Status).default("active"))
                    .col(integer(User::LoginCount).default(0))
                    .col(integer(User::FailedLogins).default(0))
                    .col(timestamp_with_time_zone_null(User::LockedUntil))
                    .col(boolean(User::IsDeleted).default(false))
                    .col(timestamp_with_time_zone(User::CreatedAt))
                    .col(timestamp_with_time_zone(User::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-org_id")
                            .from(User::Table, User::OrgId)
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
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    PersonId,
    PartyId,
    OrgId,
    Email,
    EmailVerified,
    Phone,
    PhoneVerified,
    PasswordHash,
    MfaEnabled,
    FullName,
    PrimaryRole,
    Status,
    LoginCount,
    FailedLogins,
    LockedUntil,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}
