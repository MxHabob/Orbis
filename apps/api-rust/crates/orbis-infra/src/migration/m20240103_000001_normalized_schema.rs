use sea_orm::Schema;
use sea_orm_migration::prelude::*;

use crate::models::custom_field_definition;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        // 1. Create custom_field_definitions table
        let mut custom_field_table = schema.create_table_from_entity(custom_field_definition::Entity);
        manager.create_table(custom_field_table.if_not_exists().to_owned()).await?;

        // 2. Create composite index on enrollments (student_id, period_id) if not exists
        let enroll_index = Index::create()
            .name("idx_enrollments_student_period")
            .table(Alias::new("enrollments"))
            .col(Alias::new("student_id"))
            .col(Alias::new("period_id"))
            .if_not_exists()
            .to_owned();
        manager.create_index(enroll_index).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(custom_field_definition::Entity)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
