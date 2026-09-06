use sea_orm_migration::{prelude::*, schema::*};
use super::m20240901_000002_create_user_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. user_identities Table (Google / Social / Federated identities)
        manager
            .create_table(
                Table::create()
                    .table(UserIdentities::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserIdentities::Id))
                    .col(uuid(UserIdentities::UserId))
                    .col(string(UserIdentities::Provider))
                    .col(string(UserIdentities::ProviderUserId))
                    .col(string_null(UserIdentities::Email))
                    .col(boolean(UserIdentities::IsPrimary).default(false))
                    .col(json_null(UserIdentities::ProfileData))
                    .col(timestamp_with_time_zone(UserIdentities::ConnectedAt))
                    .col(timestamp_with_time_zone_null(UserIdentities::LastUsedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_identities-user_id")
                            .from(UserIdentities::Table, UserIdentities::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-user_identities-provider-puid")
                    .table(UserIdentities::Table)
                    .col(UserIdentities::Provider)
                    .col(UserIdentities::ProviderUserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // 2. user_mfa_factors Table (TOTP / Authenticator App / WebAuthn)
        manager
            .create_table(
                Table::create()
                    .table(UserMfaFactors::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserMfaFactors::Id))
                    .col(uuid(UserMfaFactors::UserId))
                    .col(string(UserMfaFactors::FactorType).default("totp"))
                    .col(string(UserMfaFactors::FactorName))
                    .col(string(UserMfaFactors::SecretEncrypted))
                    .col(boolean(UserMfaFactors::IsVerified).default(false))
                    .col(boolean(UserMfaFactors::IsDefault).default(true))
                    .col(timestamp_with_time_zone(UserMfaFactors::CreatedAt))
                    .col(timestamp_with_time_zone_null(UserMfaFactors::LastUsedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_mfa_factors-user_id")
                            .from(UserMfaFactors::Table, UserMfaFactors::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. user_recovery_codes Table (One-time hashed recovery codes)
        manager
            .create_table(
                Table::create()
                    .table(UserRecoveryCodes::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserRecoveryCodes::Id))
                    .col(uuid(UserRecoveryCodes::UserId))
                    .col(string(UserRecoveryCodes::CodeHash))
                    .col(boolean(UserRecoveryCodes::IsUsed).default(false))
                    .col(timestamp_with_time_zone(UserRecoveryCodes::CreatedAt))
                    .col(timestamp_with_time_zone_null(UserRecoveryCodes::UsedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_recovery_codes-user_id")
                            .from(UserRecoveryCodes::Table, UserRecoveryCodes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. user_devices Table (Registered & Trusted Devices)
        manager
            .create_table(
                Table::create()
                    .table(UserDevices::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserDevices::Id))
                    .col(uuid(UserDevices::UserId))
                    .col(string(UserDevices::DeviceId))
                    .col(string(UserDevices::DeviceName))
                    .col(string(UserDevices::DeviceType).default("desktop"))
                    .col(string_null(UserDevices::Os))
                    .col(string_null(UserDevices::Browser))
                    .col(string_null(UserDevices::ClientIp))
                    .col(string_null(UserDevices::UserAgent))
                    .col(string(UserDevices::FingerprintHash))
                    .col(boolean(UserDevices::IsTrusted).default(false))
                    .col(timestamp_with_time_zone_null(UserDevices::TrustedUntil))
                    .col(timestamp_with_time_zone(UserDevices::FirstSeenAt))
                    .col(timestamp_with_time_zone(UserDevices::LastActiveAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_devices-user_id")
                            .from(UserDevices::Table, UserDevices::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-user_devices-user-device_id")
                    .table(UserDevices::Table)
                    .col(UserDevices::UserId)
                    .col(UserDevices::DeviceId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(UserDevices::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(UserRecoveryCodes::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(UserMfaFactors::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(UserIdentities::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum UserIdentities {
    Table,
    Id,
    UserId,
    Provider,
    ProviderUserId,
    Email,
    IsPrimary,
    ProfileData,
    ConnectedAt,
    LastUsedAt,
}

#[derive(DeriveIden)]
pub enum UserMfaFactors {
    Table,
    Id,
    UserId,
    FactorType,
    FactorName,
    SecretEncrypted,
    IsVerified,
    IsDefault,
    CreatedAt,
    LastUsedAt,
}

#[derive(DeriveIden)]
pub enum UserRecoveryCodes {
    Table,
    Id,
    UserId,
    CodeHash,
    IsUsed,
    CreatedAt,
    UsedAt,
}

#[derive(DeriveIden)]
pub enum UserDevices {
    Table,
    Id,
    UserId,
    DeviceId,
    DeviceName,
    DeviceType,
    Os,
    Browser,
    ClientIp,
    UserAgent,
    FingerprintHash,
    IsTrusted,
    TrustedUntil,
    FirstSeenAt,
    LastActiveAt,
}
