use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager.has_column("currencies", "symbol_position").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Currencies::Table)
                        .add_column(
                            ColumnDef::new(Currencies::SymbolPosition)
                                .string()
                                .not_null()
                                .default("after"),
                        )
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_column("currencies", "symbol_position").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Currencies::Table)
                        .drop_column(Currencies::SymbolPosition)
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}

#[derive(Iden)]
enum Currencies {
    Table,
    SymbolPosition,
}
