use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Students::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Students::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Students::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Students::PersonId).uuid().not_null())
                    .col(ColumnDef::new(Students::UserId).uuid().null())
                    .col(ColumnDef::new(Students::StudentNumber).string().not_null())
                    .col(ColumnDef::new(Students::NationalId).string().null())
                    .col(ColumnDef::new(Students::PassportNumber).string().null())
                    .col(ColumnDef::new(Students::FirstName).string().not_null())
                    .col(ColumnDef::new(Students::LastName).string().not_null())
                    .col(ColumnDef::new(Students::FullName).string().not_null())
                    .col(ColumnDef::new(Students::NameAr).string().null())
                    .col(ColumnDef::new(Students::Gender).string().not_null())
                    .col(ColumnDef::new(Students::DateOfBirth).date().null())
                    .col(ColumnDef::new(Students::PlaceOfBirth).string().null())
                    .col(ColumnDef::new(Students::Nationality).string().null())
                    .col(ColumnDef::new(Students::BloodType).string().null())
                    .col(ColumnDef::new(Students::MedicalNotes).string().null())
                    .col(ColumnDef::new(Students::SpecialNeedsNotes).string().null())
                    .col(ColumnDef::new(Students::Status).string().not_null().default("active"))
                    .col(ColumnDef::new(Students::Standing).string().not_null().default("good_standing"))
                    .col(ColumnDef::new(Students::AdmittedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Students::GraduatedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Students::IsDeleted).boolean().not_null().default(false))
                    .col(ColumnDef::new(Students::DeletedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Students::Meta).json().null())
                    .col(ColumnDef::new(Students::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Students::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_students_org")
                            .from(Students::Table, Students::OrgId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_students_person")
                            .from(Students::Table, Students::PersonId)
                            .to(Persons::Table, Persons::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_students_org_number")
                    .table(Students::Table)
                    .col(Students::OrgId)
                    .col(Students::StudentNumber)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_students_org_national_id")
                    .table(Students::Table)
                    .col(Students::OrgId)
                    .col(Students::NationalId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Students::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Students {
    Table,
    Id,
    OrgId,
    PersonId,
    UserId,
    StudentNumber,
    NationalId,
    PassportNumber,
    FirstName,
    LastName,
    FullName,
    NameAr,
    Gender,
    DateOfBirth,
    PlaceOfBirth,
    Nationality,
    BloodType,
    MedicalNotes,
    SpecialNeedsNotes,
    Status,
    Standing,
    AdmittedAt,
    GraduatedAt,
    IsDeleted,
    DeletedAt,
    Meta,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Persons {
    Table,
    Id,
}
