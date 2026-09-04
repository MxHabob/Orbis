use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StudentFees::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(StudentFees::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(StudentFees::OrgId).uuid().not_null())
                    .col(ColumnDef::new(StudentFees::StudentId).uuid().not_null())
                    .col(ColumnDef::new(StudentFees::EnrollmentId).uuid().null())
                    .col(ColumnDef::new(StudentFees::FeeTemplateId).uuid().not_null())
                    .col(ColumnDef::new(StudentFees::OriginalAmount).double().not_null())
                    .col(ColumnDef::new(StudentFees::DiscountAmount).double().not_null())
                    .col(ColumnDef::new(StudentFees::NetAmount).double().not_null())
                    .col(ColumnDef::new(StudentFees::Status).string().not_null())
                    .col(ColumnDef::new(StudentFees::DueDate).date().not_null())
                    .col(ColumnDef::new(StudentFees::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(StudentFees::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StudentFees::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StudentFees {
    Table,
    Id,
    OrgId,
    StudentId,
    EnrollmentId,
    FeeTemplateId,
    OriginalAmount,
    DiscountAmount,
    NetAmount,
    Status,
    DueDate,
    CreatedAt,
    UpdatedAt,
}
