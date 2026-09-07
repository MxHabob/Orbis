use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DiscountTemplates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DiscountTemplates::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(DiscountTemplates::OrgId).uuid().not_null())
                    .col(ColumnDef::new(DiscountTemplates::Code).string().not_null())
                    .col(ColumnDef::new(DiscountTemplates::Name).string().not_null())
                    .col(ColumnDef::new(DiscountTemplates::NameAr).string().null())
                    .col(ColumnDef::new(DiscountTemplates::DiscountType).string().not_null())
                    .col(ColumnDef::new(DiscountTemplates::Value).decimal_len(18, 4).not_null().default(0.0))
                    .col(ColumnDef::new(DiscountTemplates::EligibilityRules).json().null())
                    .col(ColumnDef::new(DiscountTemplates::RequiresApproval).boolean().not_null().default(false))
                    .col(ColumnDef::new(DiscountTemplates::IsAutoApply).boolean().not_null().default(false))
                    .col(ColumnDef::new(DiscountTemplates::IsStackable).boolean().not_null().default(false))
                    .col(ColumnDef::new(DiscountTemplates::Priority).integer().not_null().default(0))
                    .col(ColumnDef::new(DiscountTemplates::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(DiscountTemplates::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(DiscountTemplates::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_discount_templates_org")
                            .from(DiscountTemplates::Table, DiscountTemplates::OrgId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_discount_templates_org")
                    .table(DiscountTemplates::Table)
                    .col(DiscountTemplates::OrgId)
                    .col(DiscountTemplates::IsActive)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DiscountTemplates::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum DiscountTemplates {
    Table,
    Id,
    OrgId,
    Code,
    Name,
    NameAr,
    DiscountType,
    Value,
    EligibilityRules,
    RequiresApproval,
    IsAutoApply,
    IsStackable,
    Priority,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}
