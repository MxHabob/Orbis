use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Rooms::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Rooms::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Rooms::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Rooms::Name).string().not_null())
                    .col(ColumnDef::new(Rooms::Capacity).integer().not_null())
                    .col(ColumnDef::new(Rooms::RoomType).string().not_null())
                    .col(ColumnDef::new(Rooms::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Rooms::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rooms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Rooms {
    Table,
    Id,
    OrgId,
    Name,
    Capacity,
    RoomType,
    CreatedAt,
    UpdatedAt,
}
