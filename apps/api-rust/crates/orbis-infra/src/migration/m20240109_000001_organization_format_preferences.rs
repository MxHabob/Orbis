use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for column in [
            ColumnDef::new(Alias::new("format_locale"))
                .string()
                .null()
                .to_owned(),
            ColumnDef::new(Alias::new("time_format"))
                .string()
                .null()
                .to_owned(),
        ] {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("organizations"))
                        .add_column_if_not_exists(column)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("organizations"))
                    .drop_column(Alias::new("time_format"))
                    .drop_column(Alias::new("format_locale"))
                    .to_owned(),
            )
            .await
    }
}
