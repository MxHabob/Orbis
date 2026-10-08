use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager
            .has_column("organizations", "format_locale")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("organizations"))
                        .add_column(
                            ColumnDef::new(Alias::new("format_locale"))
                                .string()
                                .null(),
                        )
                        .to_owned(),
                )
                .await;
        }

        if !manager
            .has_column("organizations", "time_format")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("organizations"))
                        .add_column(
                            ColumnDef::new(Alias::new("time_format"))
                                .string()
                                .null(),
                        )
                        .to_owned(),
                )
                .await;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager
            .has_column("organizations", "time_format")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("organizations"))
                        .drop_column(Alias::new("time_format"))
                        .to_owned(),
                )
                .await;
        }
        if manager
            .has_column("organizations", "format_locale")
            .await
            .unwrap_or(false)
        {
            let _ = manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("organizations"))
                        .drop_column(Alias::new("format_locale"))
                        .to_owned(),
                )
                .await;
        }
        Ok(())
    }
}
