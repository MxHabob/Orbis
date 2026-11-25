use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::TransactionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        for statement in [
            "CREATE INDEX IF NOT EXISTS idx_journal_entries_org_date ON journal_entries (org_id, accounting_date)",
            "CREATE INDEX IF NOT EXISTS idx_journal_entries_source ON journal_entries (org_id, source_type, source_id)",
            "CREATE INDEX IF NOT EXISTS idx_journal_entry_lines_entry ON journal_entry_lines (journal_entry_id)",
        ] {
            txn.execute_unprepared(statement).await?;
        }
        txn.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        for statement in [
            "DROP INDEX IF EXISTS idx_journal_entry_lines_entry",
            "DROP INDEX IF EXISTS idx_journal_entries_source",
            "DROP INDEX IF EXISTS idx_journal_entries_org_date",
        ] {
            txn.execute_unprepared(statement).await?;
        }
        txn.commit().await
    }
}
