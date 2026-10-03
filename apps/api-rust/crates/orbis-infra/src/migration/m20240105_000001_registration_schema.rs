use sea_orm::Schema;
use sea_orm_migration::prelude::*;

use crate::models::{document_requirement, sequence_counter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        // 1. Create document_requirements table
        let mut dr_stmt = schema.create_table_from_entity(document_requirement::Entity);
        manager
            .create_table(dr_stmt.if_not_exists().to_owned())
            .await?;

        // 2. Create sequence_counters table
        let mut sc_stmt = schema.create_table_from_entity(sequence_counter::Entity);
        manager
            .create_table(sc_stmt.if_not_exists().to_owned())
            .await?;

        // 3. Indexes
        let idx_dr_org_target = Index::create()
            .name("idx_doc_requirements_org_target")
            .table(Alias::new("document_requirements"))
            .col(Alias::new("org_id"))
            .col(Alias::new("target_type"))
            .if_not_exists()
            .to_owned();
        manager.create_index(idx_dr_org_target).await?;

        let idx_sc_lookup = Index::create()
            .name("idx_sequence_counters_lookup")
            .table(Alias::new("sequence_counters"))
            .col(Alias::new("org_id"))
            .col(Alias::new("sequence_type"))
            .col(Alias::new("academic_year"))
            .unique()
            .if_not_exists()
            .to_owned();
        manager.create_index(idx_sc_lookup).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(sequence_counter::Entity)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(document_requirement::Entity)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
