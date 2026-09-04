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
                    .table(Person::Table)
                    .if_not_exists()
                    .col(pk_uuid(Person::Id))
                    .col(uuid_null(Person::OrgId))
                    .col(string(Person::FirstName))
                    .col(string(Person::LastName))
                    .col(string_null(Person::FullName))
                    .col(string_null(Person::NationalId))
                    .col(string_null(Person::Gender))
                    .col(string_null(Person::DateOfBirth))
                    .col(string_null(Person::Nationality))
                    .col(timestamp_with_time_zone(Person::CreatedAt))
                    .col(timestamp_with_time_zone(Person::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-person-org_id")
                            .from(Person::Table, Person::OrgId)
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
            .drop_table(Table::drop().table(Person::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Person {
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
