use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StudentGuardians::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(StudentGuardians::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(StudentGuardians::StudentId).uuid().not_null())
                    .col(ColumnDef::new(StudentGuardians::GuardianId).uuid().not_null())
                    .col(ColumnDef::new(StudentGuardians::Relationship).string().not_null())
                    .col(ColumnDef::new(StudentGuardians::RelationshipAr).string().null())
                    .col(ColumnDef::new(StudentGuardians::IsPrimary).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::IsEmergencyContact).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::IsAuthorizedPickup).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::CanViewGrades).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::CanViewAttendance).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::CanViewBehavior).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::CanViewFinance).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::CanViewSchedule).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::CanReceiveNotifications).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::IsPayer).boolean().not_null().default(false))
                    .col(ColumnDef::new(StudentGuardians::PaymentSharePercent).double().null())
                    .col(ColumnDef::new(StudentGuardians::Occupation).string().null())
                    .col(ColumnDef::new(StudentGuardians::Employer).string().null())
                    .col(ColumnDef::new(StudentGuardians::WorkPhone).string().null())
                    .col(ColumnDef::new(StudentGuardians::Notes).string().null())
                    .col(ColumnDef::new(StudentGuardians::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(StudentGuardians::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StudentGuardians::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StudentGuardians {
    Table,
    Id,
    StudentId,
    GuardianId,
    Relationship,
    RelationshipAr,
    IsPrimary,
    IsEmergencyContact,
    IsAuthorizedPickup,
    CanViewGrades,
    CanViewAttendance,
    CanViewBehavior,
    CanViewFinance,
    CanViewSchedule,
    CanReceiveNotifications,
    IsPayer,
    PaymentSharePercent,
    Occupation,
    Employer,
    WorkPhone,
    Notes,
    CreatedAt,
    UpdatedAt,
}
