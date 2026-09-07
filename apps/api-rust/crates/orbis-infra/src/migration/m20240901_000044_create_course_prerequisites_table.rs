use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CoursePrerequisites::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CoursePrerequisites::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(CoursePrerequisites::OrgId).uuid().not_null())
                    .col(ColumnDef::new(CoursePrerequisites::SubjectId).uuid().not_null())
                    .col(ColumnDef::new(CoursePrerequisites::PrerequisiteSubjectId).uuid().not_null())
                    .col(ColumnDef::new(CoursePrerequisites::RequirementType).string().not_null())
                    .col(ColumnDef::new(CoursePrerequisites::MinGrade).string().null())
                    .col(ColumnDef::new(CoursePrerequisites::MinScore).decimal_len(10, 2).null())
                    .col(ColumnDef::new(CoursePrerequisites::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(CoursePrerequisites::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_prereq_subject")
                            .from(CoursePrerequisites::Table, CoursePrerequisites::SubjectId)
                            .to(Subjects::Table, Subjects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_prereq_required_subject")
                            .from(CoursePrerequisites::Table, CoursePrerequisites::PrerequisiteSubjectId)
                            .to(Subjects::Table, Subjects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_prereq_subject")
                    .table(CoursePrerequisites::Table)
                    .col(CoursePrerequisites::OrgId)
                    .col(CoursePrerequisites::SubjectId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CoursePrerequisites::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum CoursePrerequisites {
    Table,
    Id,
    OrgId,
    SubjectId,
    PrerequisiteSubjectId,
    RequirementType,
    MinGrade,
    MinScore,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Subjects {
    Table,
    Id,
}
