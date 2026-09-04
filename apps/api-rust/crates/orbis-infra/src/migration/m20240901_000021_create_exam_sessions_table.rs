use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExamSessions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ExamSessions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ExamSessions::OrgId).uuid().not_null())
                    .col(ColumnDef::new(ExamSessions::AcademicPeriodId).uuid().not_null())
                    .col(ColumnDef::new(ExamSessions::Name).string().not_null())
                    .col(ColumnDef::new(ExamSessions::StartDate).date().not_null())
                    .col(ColumnDef::new(ExamSessions::EndDate).date().not_null())
                    .col(ColumnDef::new(ExamSessions::Status).string().not_null())
                    .col(ColumnDef::new(ExamSessions::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ExamSessions::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExamSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ExamSessions {
    Table,
    Id,
    OrgId,
    AcademicPeriodId,
    Name,
    StartDate,
    EndDate,
    Status,
    CreatedAt,
    UpdatedAt,
}
