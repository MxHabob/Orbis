use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TimeSlots::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TimeSlots::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(TimeSlots::PeriodId).uuid().not_null())
                    .col(ColumnDef::new(TimeSlots::DayOfWeek).small_integer().not_null())
                    .col(ColumnDef::new(TimeSlots::StartTime).time().not_null())
                    .col(ColumnDef::new(TimeSlots::EndTime).time().not_null())
                    .col(ColumnDef::new(TimeSlots::SlotIndex).integer().not_null())
                    .col(ColumnDef::new(TimeSlots::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(TimeSlots::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TimeSlots::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TimeSlots {
    Table,
    Id,
    PeriodId,
    DayOfWeek,
    StartTime,
    EndTime,
    SlotIndex,
    CreatedAt,
    UpdatedAt,
}
