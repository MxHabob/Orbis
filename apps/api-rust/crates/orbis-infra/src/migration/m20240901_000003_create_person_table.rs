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
                    .table(Persons::Table)
                    .if_not_exists()
                    .col(pk_uuid(Persons::Id))
                    .col(uuid_null(Persons::OrgId))
                    .col(string(Persons::FirstName))
                    .col(string(Persons::LastName))
                    .col(string_null(Persons::FullName))
                    .col(string_null(Persons::NationalId))
                    .col(string_null(Persons::Gender))
                    .col(string_null(Persons::DateOfBirth))
                    .col(string_null(Persons::Nationality))
                    .col(timestamp_with_time_zone(Persons::CreatedAt))
                    .col(timestamp_with_time_zone(Persons::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-person-org_id")
                            .from(Persons::Table, Persons::OrgId)
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
            .drop_table(Table::drop().table(Persons::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Persons {
    Table,
    Id,
    OrgId,
    FirstName,
    LastName,
    FullName,
    NationalId,
    Gender,
    DateOfBirth,
    Nationality,
    CreatedAt,
    UpdatedAt,
}
