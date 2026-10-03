use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create folders table if not exists
        let create_folders = Table::create()
            .table(Alias::new("folders"))
            .if_not_exists()
            .col(ColumnDef::new(Alias::new("id")).uuid().not_null().primary_key())
            .col(ColumnDef::new(Alias::new("org_id")).uuid().not_null())
            .col(ColumnDef::new(Alias::new("parent_id")).uuid().null())
            .col(ColumnDef::new(Alias::new("name")).string().not_null())
            .col(ColumnDef::new(Alias::new("path")).string().not_null())
            .col(ColumnDef::new(Alias::new("folder_type")).string().not_null().default("general"))
            .col(ColumnDef::new(Alias::new("entity_type")).string().null())
            .col(ColumnDef::new(Alias::new("entity_id")).uuid().null())
            .col(ColumnDef::new(Alias::new("org_unit_id")).uuid().null())
            .col(ColumnDef::new(Alias::new("academic_year_id")).uuid().null())
            .col(ColumnDef::new(Alias::new("term_id")).uuid().null())
            .col(ColumnDef::new(Alias::new("is_system")).boolean().not_null().default(false))
            .col(ColumnDef::new(Alias::new("metadata")).json().null())
            .col(ColumnDef::new(Alias::new("created_by")).uuid().null())
            .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null())
            .col(ColumnDef::new(Alias::new("updated_at")).date_time().not_null())
            .col(ColumnDef::new(Alias::new("deleted_at")).date_time().null())
            .to_owned();
        let _ = manager.create_table(create_folders).await;

        // 2. Indexes for folders
        let idx_folders_parent = Index::create()
            .name("idx_folders_org_parent")
            .table(Alias::new("folders"))
            .col(Alias::new("org_id"))
            .col(Alias::new("parent_id"))
            .if_not_exists()
            .to_owned();
        let _ = manager.create_index(idx_folders_parent).await;

        let idx_folders_path = Index::create()
            .name("idx_folders_org_path")
            .table(Alias::new("folders"))
            .col(Alias::new("org_id"))
            .col(Alias::new("path"))
            .if_not_exists()
            .to_owned();
        let _ = manager.create_index(idx_folders_path).await;

        let idx_folders_entity = Index::create()
            .name("idx_folders_org_entity")
            .table(Alias::new("folders"))
            .col(Alias::new("org_id"))
            .col(Alias::new("entity_type"))
            .col(Alias::new("entity_id"))
            .if_not_exists()
            .to_owned();
        let _ = manager.create_index(idx_folders_entity).await;

        let idx_folders_type = Index::create()
            .name("idx_folders_org_type")
            .table(Alias::new("folders"))
            .col(Alias::new("org_id"))
            .col(Alias::new("folder_type"))
            .if_not_exists()
            .to_owned();
        let _ = manager.create_index(idx_folders_type).await;

        // 3. Add columns to documents table
        let columns = vec![
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("org_unit_id"))
                        .uuid()
                        .null(),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("folder_id"))
                        .uuid()
                        .null(),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("academic_year_id"))
                        .uuid()
                        .null(),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("term_id"))
                        .uuid()
                        .null(),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("checksum_sha256"))
                        .string()
                        .null(),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("storage_key"))
                        .string()
                        .null(),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("is_archived"))
                        .boolean()
                        .not_null()
                        .default(false),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("archived_at"))
                        .date_time()
                        .null(),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("archived_by"))
                        .uuid()
                        .null(),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("archive_reason"))
                        .text()
                        .null(),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("is_legal_hold"))
                        .boolean()
                        .not_null()
                        .default(false),
                )
                .to_owned(),
            Table::alter()
                .table(Alias::new("documents"))
                .add_column_if_not_exists(
                    ColumnDef::new(Alias::new("deleted_by"))
                        .uuid()
                        .null(),
                )
                .to_owned(),
        ];

        for stmt in columns {
            let _ = manager.alter_table(stmt).await;
        }

        // 4. Performance Indexes on documents
        let idx_lifecycle = Index::create()
            .name("idx_documents_org_lifecycle")
            .table(Alias::new("documents"))
            .col(Alias::new("org_id"))
            .col(Alias::new("is_archived"))
            .col(Alias::new("deleted_at"))
            .if_not_exists()
            .to_owned();
        let _ = manager.create_index(idx_lifecycle).await;

        let idx_entity = Index::create()
            .name("idx_documents_org_entity")
            .table(Alias::new("documents"))
            .col(Alias::new("org_id"))
            .col(Alias::new("entity_type"))
            .col(Alias::new("entity_id"))
            .if_not_exists()
            .to_owned();
        let _ = manager.create_index(idx_entity).await;

        let idx_folder = Index::create()
            .name("idx_documents_org_folder")
            .table(Alias::new("documents"))
            .col(Alias::new("org_id"))
            .col(Alias::new("folder_id"))
            .if_not_exists()
            .to_owned();
        let _ = manager.create_index(idx_folder).await;

        let idx_academic = Index::create()
            .name("idx_documents_org_academic")
            .table(Alias::new("documents"))
            .col(Alias::new("org_id"))
            .col(Alias::new("academic_year_id"))
            .col(Alias::new("term_id"))
            .if_not_exists()
            .to_owned();
        let _ = manager.create_index(idx_academic).await;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
