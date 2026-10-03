use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240107_000001_academic_period_kind_assessment_enrichment"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ── 1. Add period_kind to academic_periods ─────────────────────
        if !manager
            .has_column("academic_periods", "period_kind")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("academic_periods"))
                        .add_column(
                            ColumnDef::new(Alias::new("period_kind"))
                                .string()
                                .not_null()
                                .default("teaching"),
                        )
                        .to_owned(),
                )
                .await;
        }

        // ── 2. Add new columns to assessment_schemes ───────────────────
        if !manager
            .has_column("assessment_schemes", "name_ar")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("assessment_schemes"))
                        .add_column(ColumnDef::new(Alias::new("name_ar")).string().null())
                        .to_owned(),
                )
                .await;
        }

        if !manager
            .has_column("assessment_schemes", "level")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("assessment_schemes"))
                        .add_column(
                            ColumnDef::new(Alias::new("level"))
                                .string()
                                .not_null()
                                .default("monthly_period"),
                        )
                        .to_owned(),
                )
                .await;
        }

        if !manager
            .has_column("assessment_schemes", "institution_scope")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("assessment_schemes"))
                        .add_column(
                            ColumnDef::new(Alias::new("institution_scope"))
                                .string()
                                .not_null()
                                .default("all"),
                        )
                        .to_owned(),
                )
                .await;
        }

        if !manager
            .has_column("assessment_schemes", "components_json")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("assessment_schemes"))
                        .add_column(
                            ColumnDef::new(Alias::new("components_json"))
                                .string()
                                .not_null()
                                .default("[]"),
                        )
                        .to_owned(),
                )
                .await;
        }

        if !manager
            .has_column("assessment_schemes", "is_default")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("assessment_schemes"))
                        .add_column(
                            ColumnDef::new(Alias::new("is_default"))
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .to_owned(),
                )
                .await;
        }

        if !manager
            .has_column("assessment_schemes", "is_system")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("assessment_schemes"))
                        .add_column(
                            ColumnDef::new(Alias::new("is_system"))
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .to_owned(),
                )
                .await;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop period_kind from academic_periods
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("academic_periods"))
                    .drop_column(Alias::new("period_kind"))
                    .to_owned(),
            )
            .await;

        // Drop new columns from assessment_schemes
        for col in &[
            "name_ar",
            "level",
            "institution_scope",
            "components_json",
            "is_default",
            "is_system",
        ] {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("assessment_schemes"))
                        .drop_column(Alias::new(*col))
                        .to_owned(),
                )
                .await;
        }

        Ok(())
    }
}
