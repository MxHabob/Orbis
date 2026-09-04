use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subjects::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Subjects::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Subjects::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Subjects::UnitId).uuid().null())
                    .col(ColumnDef::new(Subjects::Code).string().not_null())
                    .col(ColumnDef::new(Subjects::Name).string().not_null())
                    .col(ColumnDef::new(Subjects::NameAr).string().null())
                    .col(ColumnDef::new(Subjects::NameEn).string().null())
                    .col(ColumnDef::new(Subjects::Description).string().null())
                    .col(ColumnDef::new(Subjects::ShortDescription).string().null())
                    .col(ColumnDef::new(Subjects::IsCore).boolean().not_null().default(true))
                    .col(ColumnDef::new(Subjects::IsElective).boolean().not_null().default(false))
                    .col(ColumnDef::new(Subjects::CreditHours).integer().null())
                    .col(ColumnDef::new(Subjects::RequiresLab).boolean().not_null().default(false))
                    .col(ColumnDef::new(Subjects::RequiresPractical).boolean().not_null().default(false))
                    .col(ColumnDef::new(Subjects::MaxStudentsPerClass).integer().null())
                    .col(ColumnDef::new(Subjects::DefaultGradeGroups).json().null())
                    .col(ColumnDef::new(Subjects::Prerequisites).json().null())
                    .col(ColumnDef::new(Subjects::LearningOutcomes).json().null())
                    .col(ColumnDef::new(Subjects::Color).string().null())
                    .col(ColumnDef::new(Subjects::Icon).string().null())
                    .col(ColumnDef::new(Subjects::CoverImage).string().null())
                    .col(ColumnDef::new(Subjects::SortOrder).integer().null())
                    .col(ColumnDef::new(Subjects::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Subjects::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Subjects::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subjects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Subjects {
    Table,
    Id,
    OrgId,
    UnitId,
    Code,
    Name,
    NameAr,
    NameEn,
    Description,
    ShortDescription,
    IsCore,
    IsElective,
    CreditHours,
    RequiresLab,
    RequiresPractical,
    MaxStudentsPerClass,
    DefaultGradeGroups,
    Prerequisites,
    LearningOutcomes,
    Color,
    Icon,
    CoverImage,
    SortOrder,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
