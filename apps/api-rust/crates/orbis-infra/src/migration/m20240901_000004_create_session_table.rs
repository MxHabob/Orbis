use sea_orm_migration::{prelude::*, schema::*};
use super::m20240901_000002_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(pk_uuid(Session::Id))
                    .col(uuid(Session::UserId))
                    .col(string(Session::Token))
                    .col(string_null(Session::RefreshToken))
                    .col(boolean(Session::IsCurrent).default(true))
                    .col(boolean(Session::IsTrusted).default(false))
                    .col(boolean(Session::MfaVerified).default(false))
                    .col(timestamp_with_time_zone(Session::ExpiresAt))
                    .col(timestamp_with_time_zone_null(Session::RefreshExpiresAt))
                    .col(timestamp_with_time_zone_null(Session::LastActiveAt))
                    .col(timestamp_with_time_zone(Session::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-session-user_id")
                            .from(Session::Table, Session::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Session {
    Table,
    Id,
    UserId,
    Token,
    RefreshToken,
    IsCurrent,
    IsTrusted,
    MfaVerified,
    ExpiresAt,
    RefreshExpiresAt,
    LastActiveAt,
    CreatedAt,
}
