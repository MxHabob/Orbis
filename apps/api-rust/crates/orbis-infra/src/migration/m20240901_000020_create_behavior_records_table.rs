use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BehaviorRecords::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BehaviorRecords::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(BehaviorRecords::StudentId).uuid().not_null())
                    .col(ColumnDef::new(BehaviorRecords::OrgId).uuid().not_null())
                    .col(ColumnDef::new(BehaviorRecords::ReporterId).uuid().not_null())
                    .col(ColumnDef::new(BehaviorRecords::RecordType).string().not_null())
                    .col(ColumnDef::new(BehaviorRecords::Description).string().not_null())
                    .col(ColumnDef::new(BehaviorRecords::Points).integer().not_null())
                    .col(ColumnDef::new(BehaviorRecords::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(BehaviorRecords::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BehaviorRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BehaviorRecords {
    Table,
    Id,
    StudentId,
    OrgId,
    ReporterId,
    RecordType,
    Description,
    Points,
    CreatedAt,
    UpdatedAt,
}
