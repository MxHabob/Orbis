use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::models::{role, role_permission};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager
            .has_column("academic_override_requests", "approval_policy_version")
            .await?
        {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("academic_override_requests"))
                        .add_column(
                            ColumnDef::new(Alias::new("approval_policy_version"))
                                .integer()
                                .null(),
                        )
                        .to_owned(),
                )
                .await?;
        }
        manager
            .create_index(
                Index::create()
                    .name("idx_user_sessions_access_token")
                    .table(Alias::new("user_sessions"))
                    .col(Alias::new("token"))
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        let db = manager.get_connection();
        let principals = role::Entity::find()
            .filter(role::Column::Id.eq("principal"))
            .filter(role::Column::IsSystem.eq(true))
            .all(db)
            .await?;
        for principal in principals {
            let grants = role_permission::Entity::find()
                .filter(role_permission::Column::OrgId.eq(principal.org_id))
                .filter(role_permission::Column::RoleId.eq(&principal.id))
                .filter(role_permission::Column::Resource.eq("workflow"))
                .all(db)
                .await?;
            for action in ["read", "write", "approve"] {
                if grants.iter().any(|grant| grant.action == action) {
                    continue;
                }
                role_permission::ActiveModel {
                    id: Set(uuid::Uuid::new_v4()),
                    role_id: Set(principal.id.clone()),
                    org_id: Set(principal.org_id),
                    resource: Set("workflow".into()),
                    action: Set(action.into()),
                    condition: Set(None),
                    created_at: Set(chrono::Utc::now()),
                }
                .insert(db)
                .await?;
            }
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_sessions_access_token")
                    .table(Alias::new("user_sessions"))
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("academic_override_requests"))
                    .drop_column(Alias::new("approval_policy_version"))
                    .to_owned(),
            )
            .await
    }
}
