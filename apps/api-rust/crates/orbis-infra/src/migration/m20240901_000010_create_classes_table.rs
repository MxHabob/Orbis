use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Classes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Classes::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Classes::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Classes::UnitId).uuid().not_null())
                    .col(ColumnDef::new(Classes::PeriodId).uuid().not_null())
                    .col(ColumnDef::new(Classes::Name).string().not_null())
                    .col(ColumnDef::new(Classes::NameAr).string().null())
                    .col(ColumnDef::new(Classes::Level).string().not_null())
                    .col(ColumnDef::new(Classes::Division).string().null())
                    .col(ColumnDef::new(Classes::HomeroomTeacherId).uuid().null())
                    .col(ColumnDef::new(Classes::Capacity).integer().null())
                    .col(ColumnDef::new(Classes::CurrentStudents).integer().null())
                    .col(ColumnDef::new(Classes::RoomId).uuid().null())
                    .col(ColumnDef::new(Classes::Color).string().null())
                    .col(ColumnDef::new(Classes::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Classes::IsDeleted).boolean().not_null().default(false))
                    .col(ColumnDef::new(Classes::DeletedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Classes::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Classes::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Classes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Classes {
    Table,
    Id,
    OrgId,
    UnitId,
    PeriodId,
    Name,
    NameAr,
    Level,
    Division,
    HomeroomTeacherId,
    Capacity,
    CurrentStudents,
    RoomId,
    Color,
    IsActive,
    IsDeleted,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
