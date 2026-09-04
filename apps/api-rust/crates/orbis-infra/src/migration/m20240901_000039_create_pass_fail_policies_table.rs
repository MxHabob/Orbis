use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PassFailPolicies::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PassFailPolicies::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(PassFailPolicies::AssessmentSchemeId).uuid().not_null())
                    .col(ColumnDef::new(PassFailPolicies::PassScoreThreshold).double().not_null())
                    .col(ColumnDef::new(PassFailPolicies::AttendanceRequirement).double().null())
                    .col(ColumnDef::new(PassFailPolicies::RulesJson).json().null())
                    .col(ColumnDef::new(PassFailPolicies::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(PassFailPolicies::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PassFailPolicies::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PassFailPolicies {
    Table,
    Id,
    AssessmentSchemeId,
    PassScoreThreshold,
    AttendanceRequirement,
    RulesJson,
    CreatedAt,
    UpdatedAt,
}
