use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240110_000001_account_name_ar_and_subtype"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add name_ar column to accounts table if it doesn't already exist
        if !manager
            .has_column("accounts", "name_ar")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("accounts"))
                        .add_column(ColumnDef::new(Alias::new("name_ar")).string().null())
                        .to_owned(),
                )
                .await;
        }

        // Add sub_type column to accounts table if it doesn't already exist
        if !manager
            .has_column("accounts", "sub_type")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("accounts"))
                        .add_column(ColumnDef::new(Alias::new("sub_type")).string().null())
                        .to_owned(),
                )
                .await;
        }

        // Backfill standard Arabic account names for existing accounts where name_ar IS NULL
        let db = manager.get_connection();
        let standard_translations = vec![
            ("Assets", "الأصول"),
            ("Current Assets", "الأصول المتداولة"),
            ("Non-Current Assets", "الأصول غير المتداولة"),
            ("Liabilities", "الالتزامات"),
            ("Current Liabilities", "الالتزامات المتداولة"),
            ("Non-Current Liabilities", "الالتزامات غير المتداولة"),
            ("Equity", "حقوق الملكية"),
            ("Revenue", "الإيرادات"),
            ("Expenses", "المصروفات"),
            ("Cash and Banks", "النقدية والبنوك"),
            ("Cash on Hand", "صندوق النقدية"),
            ("Operating Expenses", "المصروفات التشغيلية"),
            ("Tuition Revenue", "إيرادات الرسوم الدراسية"),
            ("Student Receivables", "ذمم الطلاب المدينة"),
        ];

        for (en, ar) in standard_translations {
            let sql = format!(
                "UPDATE accounts SET name_ar = '{}' WHERE (name_ar IS NULL OR name_ar = '') AND name = '{}'",
                ar, en
            );
            let _ = db.execute_unprepared(&sql).await;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("accounts"))
                    .drop_column(Alias::new("name_ar"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("accounts"))
                    .drop_column(Alias::new("sub_type"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
