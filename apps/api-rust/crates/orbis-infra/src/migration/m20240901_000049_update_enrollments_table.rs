use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Enrollments::Table)
                    .add_column(
                        ColumnDef::new(Enrollments::StudyMode)
                            .string()
                            .not_null()
                            .default("regular"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Enrollments::Table)
                    .add_column(
                        ColumnDef::new(Enrollments::IsAttendanceExempt)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Enrollments::Table)
                    .add_column(
                        ColumnDef::new(Enrollments::IsGradeExempt)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_enrollment_org_period_mode")
                    .table(Enrollments::Table)
                    .col(Enrollments::OrgId)
                    .col(Enrollments::PeriodId)
                    .col(Enrollments::StudyMode)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Enrollments {
    Table,
    OrgId,
    PeriodId,
    StudyMode,
    IsAttendanceExempt,
    IsGradeExempt,
}
