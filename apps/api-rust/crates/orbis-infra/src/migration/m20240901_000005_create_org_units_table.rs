use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrgUnits::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OrgUnits::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(OrgUnits::OrgId).uuid().not_null())
                    .col(ColumnDef::new(OrgUnits::ParentId).uuid().null())
                    .col(ColumnDef::new(OrgUnits::Path).string().not_null())
                    .col(ColumnDef::new(OrgUnits::Depth).integer().not_null().default(0))
                    .col(ColumnDef::new(OrgUnits::Name).string().not_null())
                    .col(ColumnDef::new(OrgUnits::NameAr).string().null())
                    .col(ColumnDef::new(OrgUnits::Code).string().null())
                    .col(ColumnDef::new(OrgUnits::UnitType).string().not_null())
                    .col(ColumnDef::new(OrgUnits::HeadId).uuid().null())
                    .col(ColumnDef::new(OrgUnits::Capacity).integer().null())
                    .col(ColumnDef::new(OrgUnits::RoomNumber).string().null())
                    .col(ColumnDef::new(OrgUnits::Floor).integer().null())
                    .col(ColumnDef::new(OrgUnits::Building).string().null())
                    .col(ColumnDef::new(OrgUnits::Settings).json().null())
                    .col(ColumnDef::new(OrgUnits::LocalPolicies).json().null())
                    .col(ColumnDef::new(OrgUnits::DataIsolation).string().not_null().default("shared"))
                    .col(ColumnDef::new(OrgUnits::ChildrenCount).integer().not_null().default(0))
                    .col(ColumnDef::new(OrgUnits::DescendantCount).integer().not_null().default(0))
                    .col(ColumnDef::new(OrgUnits::SortOrder).integer().null())
                    .col(ColumnDef::new(OrgUnits::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(OrgUnits::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(OrgUnits::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrgUnits::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum OrgUnits {
    Table,
    Id,
    OrgId,
    ParentId,
    Path,
    Depth,
    Name,
    NameAr,
    Code,
    UnitType,
    HeadId,
    Capacity,
    RoomNumber,
    Floor,
    Building,
    Settings,
    LocalPolicies,
    DataIsolation,
    ChildrenCount,
    DescendantCount,
    SortOrder,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
