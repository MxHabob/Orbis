use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AiAgents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AiAgents::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AiAgents::OrgId).uuid().not_null())
                    .col(ColumnDef::new(AiAgents::Name).string().not_null())
                    .col(ColumnDef::new(AiAgents::Description).string())
                    .col(ColumnDef::new(AiAgents::ModelName).string().not_null())
                    .col(ColumnDef::new(AiAgents::Temperature).double().not_null())
                    .col(ColumnDef::new(AiAgents::SystemPrompt).text().not_null())
                    .col(ColumnDef::new(AiAgents::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(AiAgents::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AiAgents::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AiInsights::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AiInsights::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AiInsights::OrgId).uuid().not_null())
                    .col(ColumnDef::new(AiInsights::AgentId).uuid().not_null())
                    .col(ColumnDef::new(AiInsights::EntityType).string().not_null())
                    .col(ColumnDef::new(AiInsights::EntityId).uuid().not_null())
                    .col(ColumnDef::new(AiInsights::InsightType).string().not_null())
                    .col(ColumnDef::new(AiInsights::Content).text().not_null())
                    .col(ColumnDef::new(AiInsights::ConfidenceScore).double().not_null())
                    .col(ColumnDef::new(AiInsights::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AiInsights::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AiAgents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AiAgents {
    Table,
    Id,
    OrgId,
    Name,
    Description,
    ModelName,
    Temperature,
    SystemPrompt,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum AiInsights {
    Table,
    Id,
    OrgId,
    AgentId,
    EntityType,
    EntityId,
    InsightType,
    Content,
    ConfidenceScore,
    CreatedAt,
}
