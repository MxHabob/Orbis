use sea_orm_migration::prelude::*;

/// Add provenance and policy fields to assessment_schemes table.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // source_document
        manager
            .alter_table(
                Table::alter()
                    .table(AssessmentSchemes::Table)
                    .add_column(ColumnDef::new(AssessmentSchemes::SourceDocument).string().null())
                    .to_owned(),
            )
            .await?;

        // source_authority
        manager
            .alter_table(
                Table::alter()
                    .table(AssessmentSchemes::Table)
                    .add_column(ColumnDef::new(AssessmentSchemes::SourceAuthority).string().null())
                    .to_owned(),
            )
            .await?;

        // verification_status
        manager
            .alter_table(
                Table::alter()
                    .table(AssessmentSchemes::Table)
                    .add_column(ColumnDef::new(AssessmentSchemes::VerificationStatus).string().null())
                    .to_owned(),
            )
            .await?;

        // effective_date
        manager
            .alter_table(
                Table::alter()
                    .table(AssessmentSchemes::Table)
                    .add_column(ColumnDef::new(AssessmentSchemes::EffectiveDate).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;

        // policy_version (default 0)
        manager
            .alter_table(
                Table::alter()
                    .table(AssessmentSchemes::Table)
                    .add_column(
                        ColumnDef::new(AssessmentSchemes::PolicyVersion)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        // min_attendance_ratio
        manager
            .alter_table(
                Table::alter()
                    .table(AssessmentSchemes::Table)
                    .add_column(ColumnDef::new(AssessmentSchemes::MinAttendanceRatio).double().null())
                    .to_owned(),
            )
            .await?;

        // pass_threshold
        manager
            .alter_table(
                Table::alter()
                    .table(AssessmentSchemes::Table)
                    .add_column(ColumnDef::new(AssessmentSchemes::PassThreshold).double().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let columns = [
            AssessmentSchemes::SourceDocument,
            AssessmentSchemes::SourceAuthority,
            AssessmentSchemes::VerificationStatus,
            AssessmentSchemes::EffectiveDate,
            AssessmentSchemes::PolicyVersion,
            AssessmentSchemes::MinAttendanceRatio,
            AssessmentSchemes::PassThreshold,
        ];
        for col in columns {
            manager
                .alter_table(
                    Table::alter()
                        .table(AssessmentSchemes::Table)
                        .drop_column(col)
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}

#[derive(Iden)]
enum AssessmentSchemes {
    Table,
    SourceDocument,
    SourceAuthority,
    VerificationStatus,
    EffectiveDate,
    PolicyVersion,
    MinAttendanceRatio,
    PassThreshold,
}
