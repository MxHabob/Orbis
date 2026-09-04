use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Grades::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Grades::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Grades::StudentId).uuid().not_null())
                    .col(ColumnDef::new(Grades::ClassSubjectId).uuid().not_null())
                    .col(ColumnDef::new(Grades::SchemeId).uuid().not_null())
                    .col(ColumnDef::new(Grades::Score).double().not_null())
                    .col(ColumnDef::new(Grades::IsExcused).boolean().not_null().default(false))
                    .col(ColumnDef::new(Grades::Notes).string().null())
                    .col(ColumnDef::new(Grades::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Grades::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Grades::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Grades {
    Table,
    Id,
    StudentId,
    ClassSubjectId,
    SchemeId,
    Score,
    IsExcused,
    Notes,
    CreatedAt,
    UpdatedAt,
}
