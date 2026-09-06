use sea_orm_migration::{prelude::*, schema::*};
use super::m20240901_000002_create_user_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserSessions::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserSessions::Id))
                    .col(uuid(UserSessions::UserId))
                    .col(string(UserSessions::Token))
                    .col(string_null(UserSessions::RefreshToken))
                    .col(uuid_null(UserSessions::DeviceId))
                    .col(string_null(UserSessions::FamilyId))
                    .col(boolean(UserSessions::IsRevoked).default(false))
                    .col(boolean(UserSessions::IsCurrent).default(true))
                    .col(boolean(UserSessions::IsTrusted).default(false))
                    .col(boolean(UserSessions::MfaVerified).default(false))
                    .col(timestamp_with_time_zone(UserSessions::ExpiresAt))
                    .col(timestamp_with_time_zone_null(UserSessions::RefreshExpiresAt))
                    .col(timestamp_with_time_zone_null(UserSessions::LastActiveAt))
                    .col(timestamp_with_time_zone(UserSessions::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-session-user_id")
                            .from(UserSessions::Table, UserSessions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserSessions {
    Table,
    Id,
    UserId,
    Token,
    RefreshToken,
    DeviceId,
    FamilyId,
    IsRevoked,
    IsCurrent,
    IsTrusted,
    MfaVerified,
    ExpiresAt,
    RefreshExpiresAt,
    LastActiveAt,
    CreatedAt,
}
