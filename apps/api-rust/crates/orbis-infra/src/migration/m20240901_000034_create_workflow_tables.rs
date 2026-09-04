use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkflowRules::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkflowRules::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkflowRules::OrgId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowRules::Name).string().not_null())
                    .col(ColumnDef::new(WorkflowRules::Description).string())
                    .col(ColumnDef::new(WorkflowRules::TriggerEvent).string().not_null())
                    .col(ColumnDef::new(WorkflowRules::Conditions).json())
                    .col(ColumnDef::new(WorkflowRules::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(WorkflowRules::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(WorkflowRules::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WorkflowActions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkflowActions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkflowActions::RuleId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowActions::ActionType).string().not_null())
                    .col(ColumnDef::new(WorkflowActions::Config).json().not_null())
                    .col(ColumnDef::new(WorkflowActions::OrderIndex).integer().not_null().default(0))
                    .col(ColumnDef::new(WorkflowActions::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WorkflowExecutions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkflowExecutions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkflowExecutions::RuleId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowExecutions::EntityId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowExecutions::Status).string().not_null())
                    .col(ColumnDef::new(WorkflowExecutions::ErrorMsg).string())
                    .col(ColumnDef::new(WorkflowExecutions::ExecutedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(WorkflowExecutions::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(WorkflowActions::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(WorkflowRules::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum WorkflowRules {
    Table,
    Id,
    OrgId,
    Name,
    Description,
    TriggerEvent,
    Conditions,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum WorkflowActions {
    Table,
    Id,
    RuleId,
    ActionType,
    Config,
    OrderIndex,
    CreatedAt,
}

#[derive(DeriveIden)]
enum WorkflowExecutions {
    Table,
    Id,
    RuleId,
    EntityId,
    Status,
    ErrorMsg,
    ExecutedAt,
}
