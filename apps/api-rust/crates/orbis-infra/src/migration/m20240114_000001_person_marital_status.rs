use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager.has_column("persons", "marital_status").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("persons"))
                        .add_column(ColumnDef::new(Alias::new("marital_status")).string().null())
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_column("persons", "marital_status").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("persons"))
                        .drop_column(Alias::new("marital_status"))
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }
}
