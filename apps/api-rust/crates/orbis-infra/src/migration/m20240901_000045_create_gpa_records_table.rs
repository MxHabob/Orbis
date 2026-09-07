use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CumulativeGpaRecords::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CumulativeGpaRecords::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(CumulativeGpaRecords::OrgId).uuid().not_null())
                    .col(ColumnDef::new(CumulativeGpaRecords::StudentId).uuid().not_null())
                    .col(ColumnDef::new(CumulativeGpaRecords::PeriodId).uuid().not_null())
                    .col(ColumnDef::new(CumulativeGpaRecords::TermAttemptedCredits).decimal_len(10, 2).not_null().default(0.0))
                    .col(ColumnDef::new(CumulativeGpaRecords::TermEarnedCredits).decimal_len(10, 2).not_null().default(0.0))
                    .col(ColumnDef::new(CumulativeGpaRecords::TermPoints).decimal_len(10, 2).not_null().default(0.0))
                    .col(ColumnDef::new(CumulativeGpaRecords::TermGpa).decimal_len(10, 4).not_null().default(0.0))
                    .col(ColumnDef::new(CumulativeGpaRecords::CumulativeAttemptedCredits).decimal_len(10, 2).not_null().default(0.0))
                    .col(ColumnDef::new(CumulativeGpaRecords::CumulativeEarnedCredits).decimal_len(10, 2).not_null().default(0.0))
                    .col(ColumnDef::new(CumulativeGpaRecords::CumulativePoints).decimal_len(10, 2).not_null().default(0.0))
                    .col(ColumnDef::new(CumulativeGpaRecords::CumulativeGpa).decimal_len(10, 4).not_null().default(0.0))
                    .col(ColumnDef::new(CumulativeGpaRecords::Standing).string().not_null().default("good_standing"))
                    .col(ColumnDef::new(CumulativeGpaRecords::CalculatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_gpa_student_period")
                    .table(CumulativeGpaRecords::Table)
                    .col(CumulativeGpaRecords::OrgId)
                    .col(CumulativeGpaRecords::StudentId)
                    .col(CumulativeGpaRecords::PeriodId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CumulativeGpaRecords::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum CumulativeGpaRecords {
    Table,
    Id,
    OrgId,
    StudentId,
    PeriodId,
    TermAttemptedCredits,
    TermEarnedCredits,
    TermPoints,
    TermGpa,
    CumulativeAttemptedCredits,
    CumulativeEarnedCredits,
    CumulativePoints,
    CumulativeGpa,
    Standing,
    CalculatedAt,
}
