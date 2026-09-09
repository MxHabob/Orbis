use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for column in [
            Persons::NameAr,
            Persons::PassportNumber,
            Persons::PlaceOfBirth,
            Persons::BloodType,
            Persons::MedicalNotes,
            Persons::SpecialNeedsNotes,
            Persons::Phone,
            Persons::Email,
            Persons::Address,
        ] {
            manager
                .alter_table(
                    Table::alter()
                        .table(Persons::Table)
                        .add_column(ColumnDef::new(column).string().null())
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for column in [
            Persons::Address,
            Persons::Email,
            Persons::Phone,
            Persons::SpecialNeedsNotes,
            Persons::MedicalNotes,
            Persons::BloodType,
            Persons::PlaceOfBirth,
            Persons::PassportNumber,
            Persons::NameAr,
        ] {
            manager
                .alter_table(
                    Table::alter()
                        .table(Persons::Table)
                        .drop_column(column)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Persons {
    Table,
    NameAr,
    PassportNumber,
    PlaceOfBirth,
    BloodType,
    MedicalNotes,
    SpecialNeedsNotes,
    Phone,
    Email,
    Address,
}
