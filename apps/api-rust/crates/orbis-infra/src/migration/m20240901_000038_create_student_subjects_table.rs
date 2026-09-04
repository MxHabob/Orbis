use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StudentSubjects::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(StudentSubjects::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(StudentSubjects::OrgId).uuid().not_null())
                    .col(ColumnDef::new(StudentSubjects::StudentId).uuid().not_null())
                    .col(ColumnDef::new(StudentSubjects::ClassSubjectId).uuid().not_null())
                    .col(ColumnDef::new(StudentSubjects::EnrollmentId).uuid().null())
                    .col(ColumnDef::new(StudentSubjects::Status).string().not_null().default("active"))
                    .col(ColumnDef::new(StudentSubjects::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(StudentSubjects::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StudentSubjects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StudentSubjects {
    Table,
    Id,
    OrgId,
    StudentId,
    ClassSubjectId,
    EnrollmentId,
    Status,
    CreatedAt,
    UpdatedAt,
}
