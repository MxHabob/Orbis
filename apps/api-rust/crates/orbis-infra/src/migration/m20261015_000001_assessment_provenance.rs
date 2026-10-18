use sea_orm_migration::prelude::*;

/// Add provenance and policy fields to assessment_schemes table.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager
            .has_column("assessment_schemes", "source_document")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(AssessmentSchemes::Table)
                        .add_column(
                            ColumnDef::new(AssessmentSchemes::SourceDocument)
                                .string()
                                .null(),
                        )
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_column("assessment_schemes", "source_authority")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(AssessmentSchemes::Table)
                        .add_column(
                            ColumnDef::new(AssessmentSchemes::SourceAuthority)
                                .string()
                                .null(),
                        )
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_column("assessment_schemes", "verification_status")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(AssessmentSchemes::Table)
                        .add_column(
                            ColumnDef::new(AssessmentSchemes::VerificationStatus)
                                .string()
                                .null(),
                        )
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_column("assessment_schemes", "effective_date")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(AssessmentSchemes::Table)
                        .add_column(
                            ColumnDef::new(AssessmentSchemes::EffectiveDate)
                                .timestamp_with_time_zone()
                                .null(),
                        )
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_column("assessment_schemes", "policy_version")
            .await?
        {
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
        }

        if !manager
            .has_column("assessment_schemes", "min_attendance_ratio")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(AssessmentSchemes::Table)
                        .add_column(
                            ColumnDef::new(AssessmentSchemes::MinAttendanceRatio)
                                .double()
                                .null(),
                        )
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_column("assessment_schemes", "pass_threshold")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(AssessmentSchemes::Table)
                        .add_column(
                            ColumnDef::new(AssessmentSchemes::PassThreshold)
                                .double()
                                .null(),
                        )
                        .to_owned(),
                )
                .await?;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migration_handles_provenance_columns_created_by_unified_schema() {
        let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
        crate::migration::Migrator::up(&db, None).await.unwrap();
    }
}
