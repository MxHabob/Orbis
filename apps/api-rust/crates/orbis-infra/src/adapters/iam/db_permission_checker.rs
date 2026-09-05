use async_trait::async_trait;
use orbis_app::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use orbis_domain::errors::DomainError;
use orbis_app::context::RequestContext;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::models::{user_role_assignment, role_permission};

pub struct DbPermissionChecker {
    pub db: DatabaseConnection,
}

impl DbPermissionChecker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PermissionCheckerPort for DbPermissionChecker {
    async fn check(&self, ctx: &RequestContext, permission: Permission) -> Result<(), DomainError> {
        // System and Autopilot bypass permission checks completely.
        if ctx.actor.is_system() {
            return Ok(());
        }

        let actor_id = ctx.actor.id().ok_or_else(|| DomainError::Validation("Missing actor ID".into()))?;
        let perm_str = permission.as_str();

        // 1. Get all roles for the user
        let user_roles = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(actor_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if user_roles.is_empty() {
            return Err(DomainError::Unauthorized(format!("User has no roles, missing permission: {}", perm_str)));
        }

        let role_ids: Vec<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();

        // 2. Check if any of these roles have the requested permission
        let role_permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(role_ids))
            .filter(role_permission::Column::Resource.eq(perm_str))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if role_permissions.is_empty() {
            return Err(DomainError::Unauthorized(format!("Missing permission: {}", perm_str)));
        }

        Ok(())
    }

    async fn has_permission(&self, user_id: uuid::Uuid, _org_id: uuid::Uuid, permission: &str) -> Result<bool, DomainError> {
        let user_roles = user_role_assignment::Entity::find()
            .filter(user_role_assignment::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if user_roles.is_empty() {
            return Ok(false);
        }

        let role_ids: Vec<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();

        let role_permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(role_ids))
            .filter(role_permission::Column::Resource.eq(permission))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(!role_permissions.is_empty())
    }
}
