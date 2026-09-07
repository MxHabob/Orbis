use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JournalEntryLines::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(JournalEntryLines::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(JournalEntryLines::JournalEntryId).uuid().not_null())
                    .col(ColumnDef::new(JournalEntryLines::AccountId).uuid().not_null())
                    .col(ColumnDef::new(JournalEntryLines::Debit).decimal_len(18, 4).not_null().default(0.0))
                    .col(ColumnDef::new(JournalEntryLines::Credit).decimal_len(18, 4).not_null().default(0.0))
                    .col(ColumnDef::new(JournalEntryLines::Description).string().null())
                    .col(ColumnDef::new(JournalEntryLines::EntityType).string().null())
                    .col(ColumnDef::new(JournalEntryLines::EntityId).uuid().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_je_line_entry")
                            .from(JournalEntryLines::Table, JournalEntryLines::JournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_je_line_account")
                            .from(JournalEntryLines::Table, JournalEntryLines::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_je_line_entry")
                    .table(JournalEntryLines::Table)
                    .col(JournalEntryLines::JournalEntryId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(JournalEntryLines::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum JournalEntryLines {
    Table,
    Id,
    JournalEntryId,
    AccountId,
    Debit,
    Credit,
    Description,
    EntityType,
    EntityId,
}

#[derive(DeriveIden)]
enum JournalEntries {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
}
