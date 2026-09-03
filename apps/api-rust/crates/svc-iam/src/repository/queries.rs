//! IAM query repository — optimized read operations.
//!
//! These queries bypass domain services for maximum performance (CQRS).
//! They use SQL joins and projections instead of N+1 ORM queries.

use crate::entities::{user, user_session, user_role, audit_log, permission, user_permission};
use sea_orm::{entity::prelude::*, QueryFilter, QueryOrder, EntityTrait, ColumnTrait, QuerySelect};
use std::collections::HashMap;
use super::IamQueryRepo;

impl IamQueryRepo {
    // ── User Queries ────────────────────────────────────────────

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Id.eq(id))
            .filter(user::Column::IsDeleted.eq(0))
            .one(&self.db)
            .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .filter(user::Column::IsDeleted.eq(0))
            .one(&self.db)
            .await
    }

    pub async fn get_users_by_ids(&self, ids: &[String]) -> Result<HashMap<String, user::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(HashMap::new());
        }
        let users = user::Entity::find()
            .filter(user::Column::Id.is_in(ids.to_vec()))
            .filter(user::Column::IsDeleted.eq(0))
            .all(&self.db)
            .await?;
        Ok(users.into_iter().map(|u| (u.id.clone(), u)).collect())
    }

    pub async fn list_users_by_org(
        &self,
        org_id: &str,
        role: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<user::Model>, DbErr> {
        let mut query = user::Entity::find()
            .filter(user::Column::OrgId.eq(org_id))
            .filter(user::Column::IsDeleted.eq(0));

        if let Some(r) = role {
            query = query.filter(user::Column::PrimaryRole.eq(r));
        }

        query
            .order_by_asc(user::Column::FullName)
            .offset(offset)
            .limit(limit)
            .all(&self.db)
            .await
    }

    // ── Session Queries ─────────────────────────────────────────

    pub async fn get_session_by_token(&self, token: &str) -> Result<Option<user_session::Model>, DbErr> {
        user_session::Entity::find()
            .filter(user_session::Column::Token.eq(token))
            .filter(user_session::Column::IsCurrent.eq(1))
            .one(&self.db)
            .await
    }

    pub async fn get_session_by_refresh(&self, refresh_token: &str) -> Result<Option<user_session::Model>, DbErr> {
        user_session::Entity::find()
            .filter(user_session::Column::RefreshToken.eq(refresh_token))
            .one(&self.db)
            .await
    }

    // ── Role Queries ────────────────────────────────────────────

    pub async fn get_user_roles(&self, user_id: &str) -> Result<Vec<user_role::Model>, DbErr> {
        user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
    }

    // ── Permission Queries ──────────────────────────────────────

    pub async fn get_user_permissions(&self, user_id: &str) -> Result<Vec<user_permission::Model>, DbErr> {
        user_permission::Entity::find()
            .filter(user_permission::Column::UserId.eq(user_id))
            .filter(user_permission::Column::IsActive.eq(1))
            .all(&self.db)
            .await
    }

    pub async fn get_all_permissions(&self) -> Result<Vec<permission::Model>, DbErr> {
        permission::Entity::find()
            .filter(permission::Column::IsActive.eq(1))
            .all(&self.db)
            .await
    }

    // ── Audit Queries ───────────────────────────────────────────

    pub async fn get_audit_logs(
        &self,
        org_id: &str,
        limit: u64,
    ) -> Result<Vec<audit_log::Model>, DbErr> {
        audit_log::Entity::find()
            .filter(audit_log::Column::OrgId.eq(org_id))
            .order_by_desc(audit_log::Column::CreatedAt)
            .limit(limit)
            .all(&self.db)
            .await
    }
}
