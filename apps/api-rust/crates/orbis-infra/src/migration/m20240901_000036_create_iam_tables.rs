use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Roles Table
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Roles::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Roles::OrgId).uuid().not_null())
                    .col(ColumnDef::new(Roles::Name).string().not_null())
                    .col(ColumnDef::new(Roles::NameAr).string().null())
                    .col(ColumnDef::new(Roles::IsSystem).boolean().not_null().default(false))
                    .col(ColumnDef::new(Roles::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Roles::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // 2. Role Permissions Table
        manager
            .create_table(
                Table::create()
                    .table(RolePermissions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RolePermissions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(RolePermissions::RoleId).string().not_null())
                    .col(ColumnDef::new(RolePermissions::OrgId).uuid().not_null())
                    .col(ColumnDef::new(RolePermissions::Resource).string().not_null())
                    .col(ColumnDef::new(RolePermissions::Action).string().not_null())
                    .col(ColumnDef::new(RolePermissions::Condition).string().null())
                    .col(ColumnDef::new(RolePermissions::Effect).string().not_null().default("allow"))
                    .col(ColumnDef::new(RolePermissions::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // 3. User Role Assignments Table (The core of multi-tenant/multi-scope IAM)
        manager
            .create_table(
                Table::create()
                    .table(UserRoleAssignments::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRoleAssignments::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(UserRoleAssignments::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserRoleAssignments::RoleId).string().not_null())
                    .col(ColumnDef::new(UserRoleAssignments::OrgId).uuid().not_null())
                    .col(ColumnDef::new(UserRoleAssignments::ScopeType).string().not_null().default("global")) // global, branch, department
                    .col(ColumnDef::new(UserRoleAssignments::ScopeId).uuid().null())
                    .col(ColumnDef::new(UserRoleAssignments::AssignedBy).uuid().null())
                    .col(ColumnDef::new(UserRoleAssignments::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(UserRoleAssignments::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(RolePermissions::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Roles::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    OrgId,
    Name,
    NameAr,
    IsSystem,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum RolePermissions {
    Table,
    Id,
    RoleId,
    OrgId,
    Resource,
    Action,
    Condition,
    Effect,
    CreatedAt,
}

#[derive(DeriveIden)]
enum UserRoleAssignments {
    Table,
    Id,
    UserId,
    RoleId,
    OrgId,
    ScopeType,
    ScopeId,
    AssignedBy,
    CreatedAt,
}
