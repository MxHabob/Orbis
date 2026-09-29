use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240106_000001_room_floor_name_ar"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add floor column to rooms table
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("rooms"))
                    .add_column(
                        ColumnDef::new(Alias::new("floor"))
                            .integer()
                            .null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        // Add name_ar column to rooms table
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("rooms"))
                    .add_column(
                        ColumnDef::new(Alias::new("name_ar"))
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("rooms"))
                    .drop_column(Alias::new("floor"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("rooms"))
                    .drop_column(Alias::new("name_ar"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
